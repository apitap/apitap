// tracing_setup.rs
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

/// Initialize tracing subscriber with default environment-based configuration.
///
/// Reads configuration from environment variables:
/// - `APITAP_LOG_LEVEL`: Sets the log level (e.g., "info", "debug", "trace")
/// - `APITAP_LOG_FORMAT`: Set to "json" for JSON output, otherwise uses human-readable format
/// - Falls back to `RUST_LOG` if `APITAP_LOG_LEVEL` is not set
/// - Defaults to "info" level if neither is set
///
/// # Example
///
/// ```no_run
/// use apitap::log::init_tracing;
///
/// // Initialize with environment variables
/// // APITAP_LOG_LEVEL=debug APITAP_LOG_FORMAT=json
/// init_tracing();
///
/// // Now tracing is configured and ready to use
/// tracing::info!("Application started");
/// ```
pub fn init_tracing() {
    // Read from environment for backward compatibility
    let level = std::env::var("APITAP_LOG_LEVEL").ok();
    let use_json = std::env::var("APITAP_LOG_FORMAT")
        .map(|v| v.to_lowercase() == "json")
        .unwrap_or(false);
    init_tracing_with(level.as_deref(), use_json);
}

/// Initialize tracing subscriber with explicit configuration options.
///
/// Provides programmatic control over logging configuration instead of using environment variables.
///
/// # Arguments
///
/// * `level` - Optional log level string (e.g., "info", "debug", "trace").
///   If `None`, falls back to `RUST_LOG` environment variable or defaults to "info"
/// * `use_json` - If `true`, enables JSON formatter for structured logging.
///   If `false`, uses human-readable format with file/line numbers
///
/// # Example
///
/// ```no_run
/// use apitap::log::init_tracing_with;
///
/// // Initialize with debug level and human-readable format
/// init_tracing_with(Some("debug"), false);
/// tracing::debug!("Debug message visible");
///
/// // Or with JSON format for production
/// init_tracing_with(Some("info"), true);
/// tracing::info!("Structured JSON log");
/// ```
///
/// # Use Cases
///
/// - **Development**: `init_tracing_with(Some("debug"), false)` for detailed readable logs
/// - **Production**: `init_tracing_with(Some("info"), true)` for structured JSON logs
/// - **Testing**: `init_tracing_with(Some("warn"), false)` to reduce noise
pub fn init_tracing_with(level: Option<&str>, use_json: bool) {
    // Allow explicit level override, else fall back to RUST_LOG / default
    let filter = match level {
        Some(lvl) => EnvFilter::new(lvl),
        None => EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
    };

    if use_json {
        let subscriber = Registry::default()
            .with(filter)
            .with(
                fmt::layer()
                    .json()
                    .with_target(false)
                    .with_file(false)
                    .with_line_number(false),
            )
            .with(ErrorLayer::default());

        tracing::subscriber::set_global_default(subscriber)
            .expect("failed to set global tracing subscriber");
    } else {
        let subscriber = Registry::default()
            .with(filter)
            .with(
                fmt::layer()
                    .with_target(false)
                    .with_file(true)
                    .with_line_number(true),
            )
            .with(ErrorLayer::default());

        tracing::subscriber::set_global_default(subscriber)
            .expect("failed to set global tracing subscriber");
    }
}
