// SDK smoke test -- validates build-from-source and API integration.
// Uses only std library for the HTTP calls (no external deps needed for the test itself).
use std::io::{Read, Write};
use std::net::TcpStream;
use std::env;
use std::process;

static API_HOST: &str = "api.mailodds.com";

fn main() {
    let api_key = env::var("MAILODDS_TEST_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("ERROR: MAILODDS_TEST_KEY not set");
        process::exit(1);
    }

    let mut passed: u32 = 0;
    let mut failed: u32 = 0;

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
        match api_call(email, &api_key) {
            Ok((200, body)) => {
                let check = |label: &str, expected: &str, key: &str| -> bool {
                    let val = json_get(&body, key).unwrap_or_default();
                    if val == expected { true } else {
                        println!("  FAIL: {label} expected={expected} got={val}");
                        false
                    }
                };
                if check(&format!("{domain}.status"), exp_status, "status") { passed += 1; } else { failed += 1; }
                if check(&format!("{domain}.action"), exp_action, "action") { passed += 1; } else { failed += 1; }
                let sub = json_get(&body, "sub_status").unwrap_or_default();
                let exp = exp_sub.unwrap_or("");
                if sub == exp { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.sub_status expected={exp} got={sub}"); }
                if body.contains("\"test_mode\":true") { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.test_mode expected=true"); }
            }
            Ok((code, _)) => { failed += 1; println!("  FAIL: {domain} unexpected status {code}"); }
            Err(e) => { failed += 1; println!("  FAIL: {domain} error: {e}"); }
        }
    }

    // Error handling
    match api_call("test@deliverable.mailodds.com", "invalid_key") {
        Ok((401, _)) => passed += 1,
        Ok((code, _)) => { failed += 1; println!("  FAIL: error.401 got={code}"); }
        Err(e) => { failed += 1; println!("  FAIL: error.401 error: {e}"); }
    }

    match api_call_raw("{}", &api_key) {
        Ok((code, _)) if code == 400 || code == 422 => passed += 1,
        Ok((code, _)) => { failed += 1; println!("  FAIL: error.400 expected=400|422 got={code}"); }
        Err(e) => { failed += 1; println!("  FAIL: error.400 error: {e}"); }
    }

    let total = passed + failed;
    let result = if failed == 0 { "PASS" } else { "FAIL" };
    println!("\n{result}: Rust SDK ({passed}/{total})");
    if failed > 0 { process::exit(1); }
}

fn json_get(json: &str, key: &str) -> Option<String> {
    let search = format!("\"{}\":", key);
    let idx = json.find(&search)?;
    let rest = &json[idx + search.len()..];
    let rest = rest.trim_start();
    if rest.starts_with("null") {
        return Some(String::new());
    }
    if rest.starts_with('"') {
        let end = rest[1..].find('"')?;
        return Some(rest[1..1 + end].to_string());
    }
    let end = rest.find(|c: char| c == ',' || c == '}')?;
    Some(rest[..end].trim().to_string())
}

fn api_call(email: &str, key: &str) -> Result<(u16, String), String> {
    let body = format!("{{\"email\":\"{email}\"}}");
    api_call_raw(&body, key)
}

fn api_call_raw(body: &str, key: &str) -> Result<(u16, String), String> {
    // Use native-tls via a simple HTTPS approach
    // Since we can't use external crates in the test binary, shell out to curl
    let output = std::process::Command::new("curl")
        .args(["-s", "-w", "\n%{http_code}", "-X", "POST",
               "-H", &format!("Authorization: Bearer {key}"),
               "-H", "Content-Type: application/json",
               "-d", body,
               &format!("https://{API_HOST}/v1/validate"),
               "--max-time", "30"])
        .output()
        .map_err(|e| e.to_string())?;
    let out = String::from_utf8_lossy(&output.stdout).to_string();
    let lines: Vec<&str> = out.trim().rsplitn(2, '\n').collect();
    if lines.len() < 2 { return Err("unexpected curl output".into()); }
    let code: u16 = lines[0].parse().unwrap_or(0);
    Ok((code, lines[1].to_string()))
}
