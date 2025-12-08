use apitap::errors::Result;
use apitap::parse_function;
use apitap::utils::template::{
    current_date, extract_function_names, few_date_ago, substitute_env_vars, substitute_templates,
};
use chrono::Local;

#[test]
fn test_extract_function_names() -> Result<()> {
    let text = "updated > {{ currendDate() }} and updated < {{ fewDateAgo(1) }}";
    let parsed_values = extract_function_names(text)?;

    // 1) Pastikan jumlahnya 2
    assert_eq!(parsed_values.len(), 2);

    // 2) Pastikan list mengandung currendDate()
    assert!(
        parsed_values.iter().any(|v| v == "currendDate()"),
        "parsed_values should contain currendDate(), got: {:?}",
        parsed_values
    );

    // 3) Pastikan list mengandung fewDateAgo(1)
    assert!(
        parsed_values.iter().any(|v| v == "fewDateAgo(1)"),
        "parsed_values should contain fewDateAgo(1), got: {:?}",
        parsed_values
    );

    Ok(())
}

#[test]
fn test_substitute_env_vars_single() -> Result<()> {
    // Set up test environment variable
    unsafe {
        std::env::set_var("TEST_API_KEY", "secret123");
    }

    let text = "Authorization: Bearer ${TEST_API_KEY}";
    let result = substitute_env_vars(text)?;

    assert_eq!(result, "Authorization: Bearer secret123");
    assert!(!result.contains("${"));
    assert!(!result.contains("}"));

    // Clean up
    unsafe {
        std::env::remove_var("TEST_API_KEY");
    }
    Ok(())
}

#[test]
fn test_substitute_env_vars_multiple() -> Result<()> {
    // Set up test environment variables
    unsafe {
        std::env::set_var("TEST_BASE_URL", "https://api.example.com");
        std::env::set_var("TEST_API_VERSION", "v1");
        std::env::set_var("TEST_TOKEN", "token456");
    }

    let text = "${TEST_BASE_URL}/${TEST_API_VERSION}/endpoint?token=${TEST_TOKEN}";
    let result = substitute_env_vars(text)?;

    assert_eq!(
        result,
        "https://api.example.com/v1/endpoint?token=token456"
    );

    // Clean up
    unsafe {
        std::env::remove_var("TEST_BASE_URL");
        std::env::remove_var("TEST_API_VERSION");
        std::env::remove_var("TEST_TOKEN");
    }
    Ok(())
}

#[test]
fn test_substitute_env_vars_not_found() {
    // Make sure the variable doesn't exist
    unsafe {
        std::env::remove_var("NONEXISTENT_VAR_XYZ");
    }

    let text = "Value: ${NONEXISTENT_VAR_XYZ}";
    let result = substitute_env_vars(text);

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("NONEXISTENT_VAR_XYZ"));
    }
}

#[test]
fn test_substitute_env_vars_no_placeholders() -> Result<()> {
    let text = "This text has no environment variables";
    let result = substitute_env_vars(text)?;

    assert_eq!(result, text);
    Ok(())
}

#[test]
fn test_substitute_env_vars_with_templates() -> Result<()> {
    // Set up test environment variable
    unsafe {
        std::env::set_var("TEST_ENDPOINT", "users");
    }

    // Text that contains both env vars and templates (should only replace env vars)
    let text = "${TEST_ENDPOINT}?date={{ current_date() }}";
    let result = substitute_env_vars(text)?;

    // Env var should be replaced, template should remain
    assert!(result.contains("users?date={{ current_date() }}"));
    assert!(!result.contains("${TEST_ENDPOINT}"));

    // Clean up
    unsafe {
        std::env::remove_var("TEST_ENDPOINT");
    }
    Ok(())
}

#[test]
fn test_substitute_env_vars_empty_text() -> Result<()> {
    let text = "";
    let result = substitute_env_vars(text)?;

    assert_eq!(result, "");
    Ok(())
}

#[test]
fn test_substitute_env_vars_mixed_content() -> Result<()> {
    // Set up test environment variables
    unsafe {
        std::env::set_var("TEST_HOST", "localhost");
        std::env::set_var("TEST_PORT", "3000");
    }

    let text = "Server running at http://${TEST_HOST}:${TEST_PORT}/api";
    let result = substitute_env_vars(text)?;

    assert_eq!(result, "Server running at http://localhost:3000/api");

    // Clean up
    unsafe {
        std::env::remove_var("TEST_HOST");
        std::env::remove_var("TEST_PORT");
    }
    Ok(())
}

#[test]
fn test_extract_function_names_is_empty() -> Result<()> {
    let text = "asoi";
    let parsed_values = extract_function_names(text)?;

    assert!(parsed_values.is_empty());
    Ok(())
}

#[test]
fn test_current_date() {
    let expected_date = Local::now().format("%Y-%m-%d").to_string();
    let current_date_from_test = current_date();

    assert_eq!(expected_date, current_date_from_test);
}

#[test]
fn test_few_date_ago() -> Result<()> {
    let expected_date = Local::now()
        .date_naive()
        .checked_sub_signed(chrono::Duration::days(1))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    assert_eq!(expected_date, few_date_ago(1)?);
    assert!(
        few_date_ago(-1).is_err(),
        "expected error for negative days"
    );

    Ok(())
}

#[test]
fn test_macro() -> Result<()> {
    let expected_today = Local::now().format("%Y-%m-%d").to_string();
    let expected_yesterday = Local::now()
        .date_naive()
        .checked_sub_signed(chrono::Duration::days(1))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    assert_eq!(parse_function!("current_date()")?, expected_today);
    let few_date_ago_result = parse_function!("few_date_ago(1)")?;
    assert_eq!(expected_yesterday, few_date_ago_result);
    Ok(())
}

#[test]
fn test_substitute_templates() -> Result<()> {
    let expected_today = Local::now().format("%Y-%m-%d").to_string();
    let expected_yesterday = Local::now()
        .date_naive()
        .checked_sub_signed(chrono::Duration::days(1))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    let text = "lastModified>='{{ current_date() }}' AND lastModified<='{{ few_date_ago(1) }}' AND type IN (page, blogpost, comment, attachment)";
    let env_var = "${TOKEN}";
    let result = substitute_templates(text)?;
    let result2 = substitute_templates(env_var)?;

    // Should contain actual dates instead of templates
    assert!(result.contains(&expected_today));
    assert!(result.contains(&expected_yesterday));
    assert!(!result.contains("{{"));
    assert!(!result.contains("}}"));
    assert_eq!(result2,"${TOKEN}");

    Ok(())
}
