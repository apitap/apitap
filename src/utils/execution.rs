use std::{any::Any, fmt, pin::Pin, sync::Arc};

use datafusion::{
    arrow::datatypes::SchemaRef,
    common::project_schema,
    execution::{SendableRecordBatchStream, TaskContext},
    physical_expr::EquivalenceProperties,
    physical_plan::{
        stream::RecordBatchStreamAdapter, DisplayAs, DisplayFormatType, ExecutionPlan,
        Partitioning, PlanProperties,
    },
};
use futures::stream::BoxStream;
#[allow(unused_imports)]
use futures::{Stream, StreamExt, TryFutureExt};
use serde_json::Value;

use crate::{
    errors::{self},
    utils::streaming::{self, StreamConfig},
};

/// Type alias for the factory function
pub type JsonStreamFactory =
    Arc<dyn Fn() -> BoxStream<'static, errors::Result<Value>> + Send + Sync>;

#[derive(Clone)]
pub struct Exec {
    /// Factory creates a new stream each time (no Arc<Mutex> needed!)
    stream_factory: JsonStreamFactory,
    pub projected_schema: SchemaRef,
    pub cache: PlanProperties,
}

impl std::fmt::Debug for Exec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Exec")
            .field("projected_schema", &self.projected_schema)
            .field("cache", &self.cache)
            .finish()
    }
}

impl Exec {
    pub fn new<F>(
        schema: SchemaRef,
        projections: Option<&Vec<usize>>,
        stream_factory: F,
    ) -> datafusion::error::Result<Self>
    where
        F: Fn() -> Pin<Box<dyn Stream<Item = errors::Result<Value>> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let projected_schema = project_schema(&schema, projections)?;
        let cache = Self::compute_properties(projected_schema.clone());

        Ok(Self {
            stream_factory: Arc::new(stream_factory),
            projected_schema,
            cache,
        })
    }

    fn compute_properties(schema: SchemaRef) -> PlanProperties {
        let eq_properties = EquivalenceProperties::new(schema);

        PlanProperties::new(
            eq_properties,
            Partitioning::UnknownPartitioning(1),
            datafusion::physical_plan::execution_plan::EmissionType::Both,
            datafusion::physical_plan::execution_plan::Boundedness::Bounded,
        )
    }
}

impl DisplayAs for Exec {
    fn fmt_as(&self, _t: DisplayFormatType, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "HttpExec")
    }
}

impl ExecutionPlan for Exec {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema(&self) -> SchemaRef {
        self.projected_schema.clone()
    }

    fn with_new_children(
        self: Arc<Self>,
        _children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> datafusion::error::Result<Arc<dyn ExecutionPlan>> {
        Ok(self)
    }

    fn execute(
        &self,
        _partition: usize,
        _context: Arc<TaskContext>,
    ) -> datafusion::error::Result<SendableRecordBatchStream> {
        let schema = self.projected_schema.clone();
        let stream_factory = self.stream_factory.clone();
        let schema_c = schema.clone();

        // ✅ TRUE STREAMING: No intermediate buffering
        let record_batch_stream = async_stream::try_stream! {
            let json_stream = (stream_factory)();

            // ✅ Await the async function FIRST
            let batch_stream = streaming::stream_json_to_batches(
                json_stream,
                schema_c.clone(),
                StreamConfig {
                    batch_size: 256,
                    max_buffered_items: 512,
                    true_streaming: true,
                },
            )
            .await
            .map_err(|e| datafusion::error::DataFusionError::External(e.into()))?;

            let mut pinned = std::pin::pin!(batch_stream);

            while let Some(batch) = futures::StreamExt::next(&mut pinned).await {
                // ✅ Simply yield the Result as-is
                yield batch.map_err(|e| datafusion::error::DataFusionError::External(e.into()))?;
            }
        };

        let adapter = RecordBatchStreamAdapter::new(schema, record_batch_stream.boxed());
        Ok(Box::pin(adapter))
    }
    fn name(&self) -> &str {
        "Exec"
    }

    fn properties(&self) -> &datafusion::physical_plan::PlanProperties {
        &self.cache
    }

    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![]
    }
}
