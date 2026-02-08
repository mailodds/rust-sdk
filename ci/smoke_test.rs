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

    let cases: Vec<(&str, &str, &str, Option<&str>, bool, bool, bool, bool)> = vec![
        ("test@deliverable.mailodds.com", "valid", "accept", None, false, false, false, true),
        ("test@invalid.mailodds.com", "invalid", "reject", Some("smtp_rejected"), false, false, false, true),
        ("test@risky.mailodds.com", "catch_all", "accept_with_caution", Some("catch_all_detected"), false, false, false, true),
        ("test@disposable.mailodds.com", "do_not_mail", "reject", Some("disposable"), false, true, false, true),
        ("test@role.mailodds.com", "do_not_mail", "reject", Some("role_account"), false, false, true, true),
        ("test@timeout.mailodds.com", "unknown", "retry_later", Some("smtp_unreachable"), false, false, false, true),
        ("test@freeprovider.mailodds.com", "valid", "accept", None, true, false, false, true),
    ];

    for (email, exp_status, exp_action, exp_sub, exp_free, exp_disp, exp_role, exp_mx) in &cases {
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
                if resp.free_provider == *exp_free { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.free_provider expected={exp_free} got={}", resp.free_provider); }
                if resp.disposable == *exp_disp { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.disposable expected={exp_disp} got={}", resp.disposable); }
                if resp.role_account == *exp_role { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.role_account expected={exp_role} got={}", resp.role_account); }
                if resp.mx_found == *exp_mx { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.mx_found expected={exp_mx} got={}", resp.mx_found); }
                let depth_str = enum_to_value(&format!("{:?}", resp.depth));
                if depth_str == "enhanced" { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.depth expected=enhanced got={depth_str}"); }
                if !resp.processed_at.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.processed_at is empty"); }
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
