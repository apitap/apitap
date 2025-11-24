use crate::errors::{ApitapError, Result};
use datafusion::arrow::datatypes::{DataType, Field, FieldRef, Schema};
use futures::StreamExt;
use serde_arrow::schema::{SchemaLike, TracingOptions};
use serde_json::Value;
use std::collections::HashMap;
use std::{pin::Pin, sync::Arc};

/// Infer schema WITHOUT loading entire stream into memory
pub async fn infer_schema_streaming(
    mut json_stream: Pin<Box<dyn futures::Stream<Item = Result<Value>> + Send>>,
) -> Result<Arc<Schema>> {
    let mut field_types: HashMap<String, FieldInference> = HashMap::new();
    let mut samples_seen = 0;
    const MIN_SAMPLES: usize = 100; // Look at first 100 items only

    while let Some(result) = json_stream.next().await {
        let value = result?;

        if let Value::Object(obj) = value {
            for (key, val) in obj {
                let field = field_types
                    .entry(key.clone())
                    .or_insert_with(FieldInference::new);
                field.observe(&val);
            }
        }

        samples_seen += 1;
        if samples_seen >= MIN_SAMPLES {
            break; // Stop early, don't consume entire stream
        }
    }

    if field_types.is_empty() {
        return Err(ApitapError::PipelineError(
            "No fields found in JSON stream".to_string(),
        ));
    }

    let fields: Vec<Field> = field_types
        .into_iter()
        .map(|(name, inference)| {
            let data_type = inference.to_data_type();
            Field::new(name, data_type, inference.is_nullable)
        })
        .collect();

    Ok(Arc::new(Schema::new(fields)))
}

#[derive(Debug, Clone)]
struct FieldInference {
    data_type: FieldType,
    is_nullable: bool,
}

impl FieldInference {
    fn new() -> Self {
        Self {
            data_type: FieldType::Unknown,
            is_nullable: false,
        }
    }

    fn observe(&mut self, value: &Value) {
        match value {
            Value::Null => self.is_nullable = true,
            Value::Bool(_) => self.data_type = self.data_type.merge(FieldType::Boolean),
            Value::Number(n) => {
                if n.is_f64() {
                    self.data_type = self.data_type.merge(FieldType::Float64);
                } else {
                    self.data_type = self.data_type.merge(FieldType::Int64);
                }
            }
            Value::String(_) => {
                self.data_type = self.data_type.merge(FieldType::String);
            }
            Value::Array(_) => {
                // Serialize arrays as JSON strings until recursive inference is implemented
                self.data_type = self.data_type.merge(FieldType::String);
            }
            Value::Object(_) => {
                // Serialize objects as JSON strings until recursive inference is implemented
                self.data_type = self.data_type.merge(FieldType::String);
            }
        }
    }

    fn to_data_type(&self) -> DataType {
        match self.data_type {
            FieldType::Unknown => DataType::Utf8,
            FieldType::Boolean => DataType::Boolean,
            FieldType::Int64 => DataType::Int64,
            FieldType::Float64 => DataType::Float64,
            FieldType::String => DataType::Utf8,
            FieldType::List => DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))),
            FieldType::Struct => DataType::Utf8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FieldType {
    Unknown,
    Boolean,
    Int64,
    Float64,
    String,
    List,
    Struct,
}

impl FieldType {
    fn merge(self, other: FieldType) -> FieldType {
        match (self, other) {
            (Self::Unknown, t) | (t, Self::Unknown) => t,
            (Self::Boolean, Self::Boolean) => Self::Boolean,
            (Self::Int64, Self::Int64) => Self::Int64,
            (Self::Int64, Self::Float64) | (Self::Float64, Self::Int64) => Self::Float64,
            (Self::Float64, Self::Float64) => Self::Float64,
            (Self::String, _) | (_, Self::String) => Self::String,
            (Self::List, Self::List) => Self::List,
            (Self::Struct, Self::Struct) => Self::Struct,
            _ => Self::String,
        }
    }
}

/// Infer Arrow schema from a collection of JSON values
/// Preserves field order as they appear in the first JSON object
pub fn infer_schema_from_values(values: &[Value]) -> crate::errors::Result<Arc<Schema>> {
    if values.is_empty() {
        return Err(ApitapError::PipelineError(
            "No values provided for schema inference".to_string(),
        ));
    }

    // Use serde_arrow to infer schema
    let fields: Vec<FieldRef> = Vec::<FieldRef>::from_samples(
        values,
        TracingOptions::default()
            .allow_null_fields(true)
            .coerce_numbers(true)
            .map_as_struct(true), // Preserve field order from JSON
    )?;

    Ok(Arc::new(Schema::new(fields)))
}
