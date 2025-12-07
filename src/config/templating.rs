use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::errors::Result;
use minijinja::path_loader;
use minijinja::value::{Kwargs, Value};
use minijinja::{Environment, Error as MjError};
use walkdir::WalkDir;

#[derive(Debug, Default, Clone)]
pub struct RenderCapture {
    pub sink: String,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct RenderedSql {
    pub name: String,
    pub sql: String,
    pub capture: RenderCapture,
}

/// Builds a Minijinja template environment with custom functions for SQL templating.
///
/// Creates a templating environment that supports:
/// - `{{ sink(name="...") }}` - Declares the target sink/destination
/// - `{{ use_source("...") }}` - References a data source by name
///
/// The environment captures sink and source names during template rendering
/// for pipeline configuration.
///
/// # Arguments
///
/// * `root` - Root directory path for template files
/// * `shared_cap` - Shared capture state for tracking sink/source usage
///
/// # Returns
///
/// A configured `Environment` ready for SQL template rendering
///
/// # Example
///
/// ```no_run
/// use std::sync::{Arc, Mutex};
/// use apitap::config::templating::{build_env_with_captures, RenderCapture};
///
/// let capture = Arc::new(Mutex::new(RenderCapture::default()));
/// let env = build_env_with_captures("./sql", &capture);
///
/// // Environment is now ready to render SQL templates
/// // with sink() and use_source() functions
/// ```
pub fn build_env_with_captures(
    root: &str,
    shared_cap: &Arc<Mutex<RenderCapture>>,
) -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(path_loader(root));

    // {{ sink(name="...") }}
    {
        let cap = Arc::clone(shared_cap);
        env.add_function(
            "sink",
            move |kwargs: Kwargs| -> std::result::Result<Value, MjError> {
                let name: String = kwargs.get("name")?;
                let mut c = cap.lock().expect("RenderCapture mutex poisoned - this indicates a panic occurred while holding the lock");
                c.sink = name;
                Ok(Value::from(""))
            },
        );
    }

    // {{ use_source("...") }}
    {
        let cap = Arc::clone(shared_cap);
        env.add_function(
            "use_source",
            move |name: String| -> std::result::Result<Value, MjError> {
                let mut c = cap.lock().expect("RenderCapture mutex poisoned - this indicates a panic occurred while holding the lock");
                c.source = name.clone();
                Ok(Value::from(name))
            },
        );
    }

    env
}

/// Renders a single SQL template and captures metadata.
///
/// Processes a template file through Minijinja, capturing any `sink()` and
/// `use_source()` calls made during rendering. Returns the rendered SQL
/// along with captured metadata.
///
/// # Arguments
///
/// * `env` - Minijinja environment (from `build_env_with_captures`)
/// * `shared_cap` - Shared capture state to store metadata
/// * `name` - Template name/path relative to environment root
///
/// # Returns
///
/// * `Ok(RenderedSql)` - Rendered SQL with captured sink/source metadata
/// * `Err(ApitapError)` - If template not found or rendering fails
///
/// # Example
///
/// ```no_run
/// use std::sync::{Arc, Mutex};
/// use apitap::config::templating::{build_env_with_captures, render_one, RenderCapture};
///
/// let capture = Arc::new(Mutex::new(RenderCapture::default()));
/// let env = build_env_with_captures("./examples/sql", &capture);
///
/// let rendered = render_one(&env, &capture, "example.sql")
///     .expect("Failed to render template");
///
/// println!("SQL: {}", rendered.sql);
/// println!("Sink: {}", rendered.capture.sink);
/// println!("Source: {}", rendered.capture.source);
/// ```
pub fn render_one(
    env: &Environment,
    shared_cap: &Arc<Mutex<RenderCapture>>,
    name: &str,
) -> Result<RenderedSql> {
    {
        let mut c = shared_cap.lock().expect(
            "RenderCapture mutex poisoned - this indicates a panic occurred while holding the lock",
        );
        c.sink.clear();
        c.source.clear();
    }

    let tmpl = env.get_template(name)?;
    let sql = tmpl.render(())?;

    let capture = shared_cap
        .lock()
        .expect(
            "RenderCapture mutex poisoned - this indicates a panic occurred while holding the lock",
        )
        .clone();
    Ok(RenderedSql {
        name: name.to_string(),
        sql,
        capture,
    })
}

/// Lists all SQL template files in a directory recursively.
///
/// Walks through the directory tree finding all `.sql` files (case-insensitive)
/// and returns their paths relative to the root directory.
///
/// # Arguments
///
/// * `root` - Root directory to search for SQL templates
///
/// # Returns
///
/// * `Ok(Vec<String>)` - Sorted list of template paths (e.g., "example.sql", "queries/users.sql")
/// * `Err(ApitapError)` - If directory cannot be read (uses walkdir errors)
///
/// # Example
///
/// ```no_run
/// use apitap::config::templating::list_sql_templates;
///
/// let templates = list_sql_templates("./examples/sql")
///     .expect("Failed to list templates");
///
/// for template in templates {
///     println!("Found template: {}", template);
/// }
/// // Output might be:
/// // Found template: example.sql
/// // Found template: queries/users.sql
/// // Found template: transforms/aggregates.sql
/// ```
pub fn list_sql_templates(root: impl AsRef<Path>) -> Result<Vec<String>> {
    let root = root.as_ref();
    let mut out = Vec::new();

    for entry_res in WalkDir::new(root) {
        let entry = match entry_res {
            Ok(e) => e,
            Err(_) => continue,
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let is_sql = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("sql"))
            .unwrap_or(false);
        if !is_sql {
            continue;
        }

        let rel = match path.strip_prefix(root) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let name = rel
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");

        out.push(name);
    }

    out.sort();
    Ok(out)
}
