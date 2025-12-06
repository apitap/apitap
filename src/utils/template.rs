use crate::{errors::Result, ApitapError};
use chrono::{Duration, Local};
use regex::Regex;

#[macro_export]
macro_rules! parse_function {
    ($func:expr) => {{
        let input = $func;
        if input == "current_date()" {
            Ok($crate::utils::template::current_date())
        } else if input.starts_with("few_date_ago(") && input.ends_with(")") {
            let arg_str = &input[13..input.len() - 1];
            let days: i64 = arg_str.parse().map_err(|_| {
                $crate::ApitapError::PipelineError(format!("Invalid argument: {}", arg_str))
            })?;
            $crate::utils::template::few_date_ago(days)
        } else {
            Err($crate::ApitapError::PipelineError(format!(
                "Unknown function: {}",
                input
            )))
        }
    }};
}

/// Extracts function names from template strings in the format {{ function_name() }}
/// Returns only the function names without the braces.
///
/// # Example
/// ```
/// let text = "date: {{ current_date() }}";
/// let names = extract_function_names(text)?;
/// assert_eq!(names, vec!["current_date()"]);
/// ```
pub fn extract_function_names(text: &str) -> Result<Vec<String>> {
    let re = Regex::new(r"\{\{\s*([a-zA-Z_][a-zA-Z0-9_]*\([^}]*\))\s*\}\}")?;
    let data = re
        .captures_iter(text)
        .filter_map(|cap| cap.get(1))
        .map(|data| data.as_str().to_string())
        .collect::<Vec<String>>();

    Ok(data)
}

pub fn current_date() -> String {
    let now = Local::now();

    // Format jadi string, contoh: "2025-12-02"
    let formatted = now.format("%Y-%m-%d").to_string();
    formatted
}

pub fn few_date_ago(days: i64) -> Result<String> {
    if days < 0 {
        return Err(ApitapError::PipelineError(
            "days must be non-negative".to_string(),
        ));
    }

    let today = Local::now().date_naive();

    let Some(target) = today.checked_sub_signed(Duration::days(days)) else {
        return Err(ApitapError::PipelineError("date out of range".to_string()));
    };

    let final_date = target.format("%Y-%m-%d").to_string();

    Ok(final_date)
}

/// Substitutes template variables in text with their actual values.
/// Templates should be in the format {{ function_name() }}.
///
/// Supported functions:
/// - current_date(): Returns today's date in YYYY-MM-DD format
/// - few_date_ago(n): Returns date n days ago in YYYY-MM-DD format
///
/// # Example
/// ```
/// let text = "Today is {{ current_date() }}";
/// let result = substitute_templates(text)?;
/// // result: "Today is 2025-12-06"
/// ```
pub fn substitute_templates(text: &str) -> Result<String> {
    let re = Regex::new(r"\{\{\s*([a-zA-Z_][a-zA-Z0-9_]*\([^}]*\))\s*\}\}")?;

    let mut result = String::with_capacity(text.len());
    let mut last_match = 0;

    for cap in re.captures_iter(text) {
        let full_match = cap.get(0).unwrap();
        let function_name = cap.get(1).unwrap().as_str();

        // Add text before this match
        result.push_str(&text[last_match..full_match.start()]);

        // Parse and replace the function call
        let replacement_value = parse_function!(function_name)?;
        result.push_str(&replacement_value);

        last_match = full_match.end();
    }

    // Add remaining text after last match
    result.push_str(&text[last_match..]);

    Ok(result)
}
