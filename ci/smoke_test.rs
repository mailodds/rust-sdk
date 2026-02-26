// SDK smoke test -- validates build-from-source and API integration using the SDK client.
use mailodds::apis::configuration::Configuration;
use mailodds::apis::email_validation_api;
use mailodds::apis::bulk_validation_api;
use mailodds::apis::suppression_lists_api;
use mailodds::apis::validation_policies_api;
use mailodds::apis::system_api;
use mailodds::apis::sending_domains_api;
use mailodds::apis::subscriber_lists_api;
use mailodds::apis::email_sending_api;
use mailodds::models::{
    ValidateRequest, CreateJobRequest,
    AddSuppressionRequest, AddSuppressionRequestEntriesInner,
    CheckSuppressionRequest, RemoveSuppressionRequest,
    CreatePolicyFromPresetRequest, CreateSendingDomainRequest,
    CreateListRequest, SubscribeRequest,
};
use std::time::SystemTime;

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

    // ── 1. Email Validation ─────────────────────────────────────────────

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

    // ── 2. Bulk Validation ──────────────────────────────────────────────

    let create_req = CreateJobRequest::new(vec!["test@deliverable.mailodds.com".to_string()]);
    match bulk_validation_api::create_job(&config, create_req).await {
        Ok(resp) => {
            let job = resp.job.as_ref().expect("bulk.create missing job");
            let job_id = job.id.as_ref().expect("bulk.create missing job.id");
            if job_id.starts_with("job_") { passed += 1; } else { failed += 1; println!("  FAIL: bulk.create job_id prefix expected=job_ got={job_id}"); }
            let status_str = job.status.as_ref().map(|s| enum_to_value(&format!("{:?}", s))).unwrap_or_default();
            if status_str == "pending" || status_str == "processing" || status_str == "completed" {
                passed += 1;
            } else {
                failed += 1; println!("  FAIL: bulk.create status expected=pending|processing|completed got={status_str}");
            }

            // Get job
            match bulk_validation_api::get_job(&config, job_id).await {
                Ok(get_resp) => {
                    let get_job = get_resp.job.as_ref().expect("bulk.get missing job");
                    let get_id = get_job.id.as_ref().expect("bulk.get missing job.id");
                    if get_id == job_id { passed += 1; } else { failed += 1; println!("  FAIL: bulk.get job_id mismatch expected={job_id} got={get_id}"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: bulk.get error: {e}"); }
            }

            // List jobs
            match bulk_validation_api::list_jobs(&config, None, None, None).await {
                Ok(list_resp) => {
                    let jobs = list_resp.jobs.unwrap_or_default();
                    if !jobs.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: bulk.list returned empty jobs"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: bulk.list error: {e}"); }
            }

            // Delete job
            match bulk_validation_api::delete_job(&config, job_id).await {
                Ok(_) => { passed += 1; }
                Err(e) => { failed += 1; println!("  FAIL: bulk.delete error: {e}"); }
            }
        }
        Err(e) => { failed += 1; println!("  FAIL: bulk.create error: {e}"); }
    }

    // ── 3. Suppression Lists ────────────────────────────────────────────

    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string();
    let supp_email = format!("smoke-rust-{ts}@example.com");

    // Add suppression
    let entry = AddSuppressionRequestEntriesInner::new(
        mailodds::models::add_suppression_request_entries_inner::Type::Email,
        supp_email.clone(),
    );
    let add_req = AddSuppressionRequest::new(vec![entry]);
    match suppression_lists_api::add_suppression(&config, add_req).await {
        Ok(resp) => {
            let added = resp.added.unwrap_or(0);
            if added >= 1 { passed += 1; } else { failed += 1; println!("  FAIL: suppression.add added expected>=1 got={added}"); }
        }
        Err(e) => { failed += 1; println!("  FAIL: suppression.add error: {e}"); }
    }

    // Check suppression (should be suppressed)
    let check_req = CheckSuppressionRequest::new(supp_email.clone());
    match suppression_lists_api::check_suppression(&config, check_req).await {
        Ok(resp) => {
            let suppressed = resp.suppressed.unwrap_or(false);
            if suppressed { passed += 1; } else { failed += 1; println!("  FAIL: suppression.check expected=true got=false"); }
        }
        Err(e) => { failed += 1; println!("  FAIL: suppression.check error: {e}"); }
    }

    // Get suppression stats
    match suppression_lists_api::get_suppression_stats(&config).await {
        Ok(resp) => {
            let total = resp.total.unwrap_or(-1);
            if total >= 0 { passed += 1; } else { failed += 1; println!("  FAIL: suppression.stats total expected>=0 got={total}"); }
        }
        Err(e) => { failed += 1; println!("  FAIL: suppression.stats error: {e}"); }
    }

    // Remove suppression
    let remove_req = RemoveSuppressionRequest::new(vec![supp_email.clone()]);
    match suppression_lists_api::remove_suppression(&config, remove_req).await {
        Ok(_) => { passed += 1; }
        Err(e) => { failed += 1; println!("  FAIL: suppression.remove error: {e}"); }
    }

    // Check after removal (should not be suppressed)
    let check_req2 = CheckSuppressionRequest::new(supp_email.clone());
    match suppression_lists_api::check_suppression(&config, check_req2).await {
        Ok(resp) => {
            let suppressed = resp.suppressed.unwrap_or(true);
            if !suppressed { passed += 1; } else { failed += 1; println!("  FAIL: suppression.check_after_remove expected=false got=true"); }
        }
        Err(e) => { failed += 1; println!("  FAIL: suppression.check_after_remove error: {e}"); }
    }

    // ── 4. Validation Policies ──────────────────────────────────────────

    // Cleanup leftover smoke policies (free plan allows only 1)
    if let Ok(existing) = validation_policies_api::list_policies(&config, Some(true)).await {
        for p in existing.policies.unwrap_or_default() {
            if let Some(name) = &p.name {
                if name.starts_with("smoke") {
                    let _ = validation_policies_api::delete_policy(&config, p.id.unwrap_or(0)).await;
                }
            }
        }
    }

    // List presets
    let mut preset_found = false;
    match validation_policies_api::get_policy_presets(&config).await {
        Ok(resp) => {
            let presets = resp.presets.unwrap_or_default();
            if !presets.is_empty() {
                passed += 1;
                preset_found = true;
            } else {
                failed += 1; println!("  FAIL: policies.presets returned empty");
            }
        }
        Err(e) => { failed += 1; println!("  FAIL: policies.presets error: {e}"); }
    }

    // Create policy from preset + delete
    if preset_found {
        let policy_name = format!("smoke-rust-{ts}");
        let preset_req = CreatePolicyFromPresetRequest::new(
            mailodds::models::create_policy_from_preset_request::PresetId::Strict,
            policy_name.clone(),
        );
        match validation_policies_api::create_policy_from_preset(&config, preset_req).await {
            Ok(resp) => {
                let policy = resp.policy.as_ref().expect("policies.create missing policy");
                let policy_id = policy.id.expect("policies.create missing policy.id");
                let name = policy.name.as_ref().expect("policies.create missing policy.name");
                if name == &policy_name { passed += 1; } else { failed += 1; println!("  FAIL: policies.create name expected={policy_name} got={name}"); }

                // List policies
                match validation_policies_api::list_policies(&config, Some(true)).await {
                    Ok(list_resp) => {
                        let policies = list_resp.policies.unwrap_or_default();
                        if !policies.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: policies.list returned empty"); }
                    }
                    Err(e) => { failed += 1; println!("  FAIL: policies.list error: {e}"); }
                }

                // Delete policy
                match validation_policies_api::delete_policy(&config, policy_id).await {
                    Ok(resp) => {
                        let deleted = resp.deleted.unwrap_or(false);
                        if deleted { passed += 1; } else { failed += 1; println!("  FAIL: policies.delete deleted expected=true got=false"); }
                    }
                    Err(e) => { failed += 1; println!("  FAIL: policies.delete error: {e}"); }
                }
            }
            Err(e) => { failed += 1; println!("  FAIL: policies.create error: {e}"); }
        }
    }

    // ── 5. System ───────────────────────────────────────────────────────

    // Health check (no auth required)
    let mut noauth_config = Configuration::new();
    noauth_config.base_path = "https://api.mailodds.com".to_owned();
    match system_api::health_check(&noauth_config).await {
        Ok(resp) => {
            let status = resp.status.unwrap_or_default();
            if status == "ok" || status == "healthy" { passed += 1; } else { failed += 1; println!("  FAIL: system.health status expected=ok|healthy got={status}"); }
        }
        Err(e) => { failed += 1; println!("  FAIL: system.health error: {e}"); }
    }

    // Telemetry summary
    match system_api::get_telemetry_summary(&config, Some("24h")).await {
        Ok(resp) => {
            if resp.window.is_some() { passed += 1; } else { failed += 1; println!("  FAIL: system.telemetry window is None"); }
            if resp.totals.is_some() { passed += 1; } else { failed += 1; println!("  FAIL: system.telemetry totals is None"); }
        }
        Err(e) => { failed += 1; println!("  FAIL: system.telemetry error: {e}"); }
    }

    // ── 6. Sending Domains ──────────────────────────────────────────────

    // List sending domains
    match sending_domains_api::list_sending_domains(&config).await {
        Ok(resp) => {
            let domains = resp.domains.unwrap_or_default();
            // Account may or may not have domains; just verify the call succeeds
            passed += 1;
            let _ = domains; // suppress unused warning
        }
        Err(e) => { failed += 1; println!("  FAIL: domains.list error: {e}"); }
    }

    // Create + delete sending domain
    let domain_name = format!("smoke-rust-{ts}.example.com");
    let domain_req = CreateSendingDomainRequest::new(domain_name.clone());
    match sending_domains_api::create_sending_domain(&config, domain_req).await {
        Ok(resp) => {
            let domain = resp.domain.as_ref().expect("domains.create missing domain");
            let domain_id = domain.id.as_ref().expect("domains.create missing domain.id");
            let got_domain = domain.domain.as_ref().expect("domains.create missing domain.domain");
            if got_domain == &domain_name { passed += 1; } else { failed += 1; println!("  FAIL: domains.create domain expected={domain_name} got={got_domain}"); }

            // Delete sending domain
            match sending_domains_api::delete_sending_domain(&config, domain_id).await {
                Ok(_) => { passed += 1; }
                Err(e) => { failed += 1; println!("  FAIL: domains.delete error: {e}"); }
            }
        }
        Err(e) => { failed += 1; println!("  FAIL: domains.create error: {e}"); }
    }

    // ── 7. Subscriber Lists ────────────────────────────────────────────

    let list_name = format!("smoke-rust-{ts}");
    let list_req = CreateListRequest::new(list_name.clone());
    match subscriber_lists_api::create_list(&config, list_req).await {
        Ok(resp) => {
            let list = resp.list.as_ref().expect("lists.create missing list");
            let list_id = list.id.as_ref().expect("lists.create missing list.id");
            let got_name = list.name.as_ref().expect("lists.create missing list.name");
            if got_name == &list_name { passed += 1; } else { failed += 1; println!("  FAIL: lists.create name expected={list_name} got={got_name}"); }

            // List all lists
            match subscriber_lists_api::get_lists(&config, None, None).await {
                Ok(list_resp) => {
                    let lists = list_resp.lists.unwrap_or_default();
                    if !lists.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: lists.list returned empty"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: lists.list error: {e}"); }
            }

            // Subscribe
            let sub_email = format!("smoke-rust-sub-{ts}@example.com");
            let sub_req = SubscribeRequest::new(sub_email.clone());
            match subscriber_lists_api::subscribe(&config, list_id, sub_req).await {
                Ok(_) => { passed += 1; }
                Err(e) => { failed += 1; println!("  FAIL: lists.subscribe error: {e}"); }
            }

            // Get subscribers
            match subscriber_lists_api::get_subscribers(&config, list_id, None, None, None).await {
                Ok(sub_resp) => {
                    let subscribers = sub_resp.subscribers.unwrap_or_default();
                    if !subscribers.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: lists.subscribers returned empty"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: lists.subscribers error: {e}"); }
            }

            // Delete list
            match subscriber_lists_api::delete_list(&config, list_id).await {
                Ok(_) => { passed += 1; }
                Err(e) => { failed += 1; println!("  FAIL: lists.delete error: {e}"); }
            }
        }
        Err(e) => { failed += 1; println!("  FAIL: lists.create error: {e}"); }
    }

    // ── 8. Email Sending (import-only) ──────────────────────────────────
    // Verify the module compiles and types are accessible without calling
    // the send endpoint (which requires a verified sending domain).
    // Construct request objects to prove the types resolve at compile time.

    let _deliver_req = mailodds::models::DeliverRequest::new(
        vec![],
        "sender@example.com".to_string(),
        "subject".to_string(),
        "domain-id".to_string(),
    );
    let _batch_req = mailodds::models::BatchDeliverRequest::new(
        vec![],
        "sender@example.com".to_string(),
        "subject".to_string(),
        "domain-id".to_string(),
    );
    // Reference the module to prove it compiles (suppresses unused import warning).
    let _ = email_sending_api::deliver_email;
    let _ = email_sending_api::deliver_batch;
    passed += 1; // import-only check

    // ── Summary ─────────────────────────────────────────────────────────

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
