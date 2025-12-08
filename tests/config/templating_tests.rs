use apitap::config::templating::{
    build_env_with_captures, list_sql_templates, render_one, RenderCapture,
};
use std::fs;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;

#[test]
fn test_build_env_with_captures() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_str().unwrap();
    let shared_cap = Arc::new(Mutex::new(RenderCapture::default()));

    let env = build_env_with_captures(root, &shared_cap);

    // Verify environment is created successfully
    assert!(env.get_template("nonexistent.sql").is_err());
}

#[test]
fn test_sink_function_captures_name() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_str().unwrap();

    // Create a test SQL file with sink() call
    let sql_content = r#"{{ sink(name="postgres_target") }}
SELECT * FROM users;
"#;
    fs::write(temp_dir.path().join("test.sql"), sql_content).unwrap();

    let shared_cap = Arc::new(Mutex::new(RenderCapture::default()));
    let env = build_env_with_captures(root, &shared_cap);

    let result = render_one(&env, &shared_cap, "test.sql").unwrap();

    assert_eq!(result.capture.sink, "postgres_target");
    assert!(result.sql.contains("SELECT * FROM users"));
}

#[test]
fn test_use_source_function_captures_name() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_str().unwrap();

    // Create a test SQL file with use_source() call
    let sql_content = r#"{{ use_source("api_users") }}
{{ sink(name="postgres_target") }}
"#;
    fs::write(temp_dir.path().join("test.sql"), sql_content).unwrap();

    let shared_cap = Arc::new(Mutex::new(RenderCapture::default()));
    let env = build_env_with_captures(root, &shared_cap);

    let result = render_one(&env, &shared_cap, "test.sql").unwrap();

    assert_eq!(result.capture.source, "api_users");
    assert_eq!(result.capture.sink, "postgres_target");
}

#[test]
fn test_schedule_function_captures_name() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_str().unwrap();

    // Create a test SQL file with schedule() call
    let sql_content = r#"{{ schedule("daily_job") }}
{{ sink(name="postgres_target") }}
SELECT * FROM scheduled_data;
"#;
    fs::write(temp_dir.path().join("test.sql"), sql_content).unwrap();

    let shared_cap = Arc::new(Mutex::new(RenderCapture::default()));
    let env = build_env_with_captures(root, &shared_cap);

    let result = render_one(&env, &shared_cap, "test.sql").unwrap();

    assert_eq!(result.capture.source, "daily_job");
    assert_eq!(result.capture.sink, "postgres_target");
    assert!(result.sql.contains("SELECT * FROM scheduled_data"));
}

#[test]
fn test_render_one_clears_previous_captures() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_str().unwrap();

    // First template
    let sql_content1 = r#"{{ sink(name="sink1") }}
{{ use_source("source1") }}
"#;
    fs::write(temp_dir.path().join("test1.sql"), sql_content1).unwrap();

    // Second template without sink/source
    let sql_content2 = "SELECT 1;";
    fs::write(temp_dir.path().join("test2.sql"), sql_content2).unwrap();

    let shared_cap = Arc::new(Mutex::new(RenderCapture::default()));
    let env = build_env_with_captures(root, &shared_cap);

    // Render first
    let result1 = render_one(&env, &shared_cap, "test1.sql").unwrap();
    assert_eq!(result1.capture.sink, "sink1");
    assert_eq!(result1.capture.source, "source1");

    // Render second - captures should be cleared
    let result2 = render_one(&env, &shared_cap, "test2.sql").unwrap();
    assert_eq!(result2.capture.sink, "");
    assert_eq!(result2.capture.source, "");
}

#[test]
fn test_list_sql_templates_finds_all_sql_files() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create SQL files
    fs::write(root.join("query1.sql"), "SELECT 1;").unwrap();
    fs::write(root.join("query2.sql"), "SELECT 2;").unwrap();

    // Create subdirectory with SQL file
    let subdir = root.join("subdir");
    fs::create_dir(&subdir).unwrap();
    fs::write(subdir.join("query3.sql"), "SELECT 3;").unwrap();

    // Create non-SQL file (should be ignored)
    fs::write(root.join("readme.txt"), "Not SQL").unwrap();

    let templates = list_sql_templates(root).unwrap();

    assert_eq!(templates.len(), 3);
    assert!(templates.contains(&"query1.sql".to_string()));
    assert!(templates.contains(&"query2.sql".to_string()));
    assert!(templates.contains(&"subdir/query3.sql".to_string()));
}

#[test]
fn test_list_sql_templates_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    let templates = list_sql_templates(root).unwrap();

    assert_eq!(templates.len(), 0);
}

#[test]
fn test_list_sql_templates_case_insensitive() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create SQL files with different case extensions
    fs::write(root.join("query1.sql"), "SELECT 1;").unwrap();
    fs::write(root.join("query2.SQL"), "SELECT 2;").unwrap();
    fs::write(root.join("query3.Sql"), "SELECT 3;").unwrap();

    let templates = list_sql_templates(root).unwrap();

    assert_eq!(templates.len(), 3);
}

#[test]
fn test_list_sql_templates_sorted() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Create files in non-alphabetical order
    fs::write(root.join("zebra.sql"), "SELECT 1;").unwrap();
    fs::write(root.join("apple.sql"), "SELECT 2;").unwrap();
    fs::write(root.join("banana.sql"), "SELECT 3;").unwrap();

    let templates = list_sql_templates(root).unwrap();

    assert_eq!(templates[0], "apple.sql");
    assert_eq!(templates[1], "banana.sql");
    assert_eq!(templates[2], "zebra.sql");
}

#[test]
fn test_rendered_sql_contains_name() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_str().unwrap();

    let sql_content = "SELECT * FROM table;";
    fs::write(temp_dir.path().join("myquery.sql"), sql_content).unwrap();

    let shared_cap = Arc::new(Mutex::new(RenderCapture::default()));
    let env = build_env_with_captures(root, &shared_cap);

    let result = render_one(&env, &shared_cap, "myquery.sql").unwrap();

    assert_eq!(result.name, "myquery.sql");
    assert_eq!(result.sql.trim(), sql_content);
}

#[test]
fn test_render_one_with_template_variables() {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path().to_str().unwrap();

    // Template with variable (though we're not passing context)
    let sql_content = "SELECT * FROM users LIMIT 10;";
    fs::write(temp_dir.path().join("test.sql"), sql_content).unwrap();

    let shared_cap = Arc::new(Mutex::new(RenderCapture::default()));
    let env = build_env_with_captures(root, &shared_cap);

    let result = render_one(&env, &shared_cap, "test.sql").unwrap();

    assert!(result.sql.contains("LIMIT 10"));
}
