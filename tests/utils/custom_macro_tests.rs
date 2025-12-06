use apitap::errors::Result;
use apitap::parse_function;
use apitap::utils::template::{
    current_date, extract_function_names, few_date_ago, substitute_templates,
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
    let result = substitute_templates(text)?;

    // Should contain actual dates instead of templates
    assert!(result.contains(&expected_today));
    assert!(result.contains(&expected_yesterday));
    assert!(!result.contains("{{"));
    assert!(!result.contains("}}"));

    Ok(())
}
