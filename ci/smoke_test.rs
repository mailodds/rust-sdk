// SDK smoke test -- validates build-from-source and API integration using the SDK client.
use mailodds::apis::configuration::Configuration;
use mailodds::apis::email_validation_api;
use mailodds::models::ValidateRequest;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let api_key = std::env::var("MAILODDS_TEST_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("ERROR: MAILODDS_TEST_KEY not set");
        std::process::exit(1);
    }

    let mut passed: u32 = 0;
    let mut failed: u32 = 0;

    let mut config = Configuration::new();
    config.base_path = "https://api.mailodds.com".to_owned();
    config.bearer_access_token = Some(api_key.clone());

    let cases: Vec<(&str, &str, &str, Option<&str>)> = vec![
        ("test@deliverable.mailodds.com", "valid", "accept", None),
        ("test@invalid.mailodds.com", "invalid", "reject", Some("smtp_rejected")),
        ("test@risky.mailodds.com", "catch_all", "accept_with_caution", Some("catch_all_detected")),
        ("test@disposable.mailodds.com", "do_not_mail", "reject", Some("disposable")),
        ("test@role.mailodds.com", "do_not_mail", "reject", Some("role_account")),
        ("test@timeout.mailodds.com", "unknown", "retry_later", Some("smtp_unreachable")),
        ("test@freeprovider.mailodds.com", "valid", "accept", None),
    ];

    for (email, exp_status, exp_action, exp_sub) in &cases {
        let domain: &str = email.split('@').nth(1).unwrap().split('.').next().unwrap();
        let req = ValidateRequest::new(email.to_string());
        match email_validation_api::validate_email(&config, req).await {
            Ok(resp) => {
                let status = format!("{:?}", resp.status);
                let action = format!("{:?}", resp.action);
                // Convert enum Debug format to snake_case value
                let status_str = enum_to_value(&status);
                let action_str = enum_to_value(&action);
                if &status_str == exp_status { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.status expected={exp_status} got={status_str}"); }
                if &action_str == exp_action { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.action expected={exp_action} got={action_str}"); }
                let sub = resp.sub_status.as_ref().map(|s| enum_to_value(&format!("{:?}", s))).unwrap_or_default();
                let exp = exp_sub.unwrap_or("");
                if sub == exp { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.sub_status expected={exp} got={sub}"); }
            }
            Err(e) => { failed += 1; println!("  FAIL: {domain} error: {e}"); }
        }
    }

    // Error handling: 401 with bad key
    let mut bad_config = Configuration::new();
    bad_config.base_path = "https://api.mailodds.com".to_owned();
    bad_config.bearer_access_token = Some("invalid_key".to_owned());
    let req401 = ValidateRequest::new("test@deliverable.mailodds.com".to_string());
    match email_validation_api::validate_email(&bad_config, req401).await {
        Ok(_) => { failed += 1; println!("  FAIL: error.401 no error raised"); }
        Err(mailodds::apis::Error::ResponseError(content)) => {
            if content.status == 401 { passed += 1; }
            else { failed += 1; println!("  FAIL: error.401 expected=401 got={}", content.status); }
        }
        Err(e) => { failed += 1; println!("  FAIL: error.401 wrong error: {e}"); }
    }

    // Error handling: 400/422 with empty email
    let req_empty = ValidateRequest::new("".to_string());
    match email_validation_api::validate_email(&config, req_empty).await {
        Ok(_) => { failed += 1; println!("  FAIL: error.400 no error raised"); }
        Err(mailodds::apis::Error::ResponseError(content)) => {
            if content.status == 400 || content.status == 422 { passed += 1; }
            else { failed += 1; println!("  FAIL: error.400 expected=400|422 got={}", content.status); }
        }
        Err(e) => { failed += 1; println!("  FAIL: error.400 wrong error: {e}"); }
    }

    let total = passed + failed;
    let result = if failed == 0 { "PASS" } else { "FAIL" };
    println!("\n{result}: Rust SDK ({passed}/{total})");
    if failed > 0 { std::process::exit(1); }
}

fn enum_to_value(debug_str: &str) -> String {
    // Convert PascalCase enum variant to snake_case value
    // e.g., "Valid" -> "valid", "CatchAll" -> "catch_all", "AcceptWithCaution" -> "accept_with_caution"
    let mut result = String::new();
    for (i, c) in debug_str.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}
