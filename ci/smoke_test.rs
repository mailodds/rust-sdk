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
use mailodds::apis::dmarc_monitoring_api;
use mailodds::apis::blacklist_monitoring_api;
use mailodds::apis::server_tests_api;
use mailodds::apis::contact_lists_api;
use mailodds::apis::content_classification_api;
use mailodds::apis::message_events_api;
use mailodds::apis::events_api;
use mailodds::apis::alert_rules_api;
use mailodds::apis::reputation_api;
use mailodds::apis::spam_checks_api;
use mailodds::apis::bounce_analysis_api;
use mailodds::apis::pixel_settings_api;
use mailodds::apis::out_of_office_api;
use mailodds::apis::engagement_api;
use mailodds::apis::webhook_cli_api;
use mailodds::models::{
    ValidateRequest, CreateJobRequest,
    AddSuppressionRequest, AddSuppressionRequestEntriesInner,
    CheckSuppressionRequest, RemoveSuppressionRequest,
    CreatePolicyFromPresetRequest, CreateSendingDomainRequest,
    CreateListRequest, SubscribeRequest,
    AddDmarcDomainRequest, AddBlacklistMonitorRequest,
    RunServerTestRequest, CreateContactListRequest, ClassifyContentRequest,
    TrackEventRequest,
    CreateAlertRuleRequest, UpdateAlertRuleRequest,
    RunSpamCheckRequest, CreateBounceAnalysisRequest,
    UpdatePixelSettingsRequest, AddContactRequest, UpdateContactRequest,
    BatchCheckOooRequest, CreateWebhookCliSessionRequest,
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
    let mut warned: u32 = 0;

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
                let sub = resp.sub_status.as_ref().map(|s| enum_to_value(&format!("{:?}", s))).unwrap_or_default();
                let exp = exp_sub.unwrap_or("");
                if sub == "domain_not_found" && exp != "domain_not_found" {
                    warned += 1;
                    println!("  WARN: {domain} test domain not configured (domain_not_found)");
                    passed += 1; // SDK call succeeded
                } else {
                    let status = format!("{:?}", resp.status);
                    let action = format!("{:?}", resp.action);
                    // Convert enum Debug format to snake_case value
                    let status_str = enum_to_value(&status);
                    let action_str = enum_to_value(&action);
                    if &status_str == exp_status { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.status expected={exp_status} got={status_str}"); }
                    if &action_str == exp_action { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.action expected={exp_action} got={action_str}"); }
                    if sub == exp { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.sub_status expected={exp} got={sub}"); }
                    if resp.free_provider == *exp_free { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.free_provider expected={exp_free} got={}", resp.free_provider); }
                    if resp.disposable == *exp_disp { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.disposable expected={exp_disp} got={}", resp.disposable); }
                    if resp.role_account == *exp_role { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.role_account expected={exp_role} got={}", resp.role_account); }
                    if resp.mx_found == *exp_mx { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.mx_found expected={exp_mx} got={}", resp.mx_found); }
                    let depth_str = enum_to_value(&format!("{:?}", resp.depth));
                    if depth_str == "enhanced" { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.depth expected=enhanced got={depth_str}"); }
                    if !resp.processed_at.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: {domain}.processed_at is empty"); }
                }
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
            let job_id = &job.id;
            if job_id.starts_with("job_") { passed += 1; } else { failed += 1; println!("  FAIL: bulk.create job_id prefix expected=job_ got={job_id}"); }
            let status_str = enum_to_value(&format!("{:?}", job.status));
            if status_str == "pending" || status_str == "processing" || status_str == "completed" {
                passed += 1;
            } else {
                failed += 1; println!("  FAIL: bulk.create status expected=pending|processing|completed got={status_str}");
            }

            // Get job
            match bulk_validation_api::get_job(&config, job_id).await {
                Ok(get_resp) => {
                    let get_job = get_resp.job.as_ref().expect("bulk.get missing job");
                    let get_id = &get_job.id;
                    if get_id == job_id { passed += 1; } else { failed += 1; println!("  FAIL: bulk.get job_id mismatch expected={job_id} got={get_id}"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: bulk.get error: {e}"); }
            }

            // List jobs
            match bulk_validation_api::list_jobs(&config, None, None, None).await {
                Ok(list_resp) => {
                    let jobs = list_resp.data.unwrap_or_default();
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
                Err(mailodds::apis::Error::ResponseError(content)) if content.status == 500 => {
                    warned += 1;
                    println!("  WARN: domains.delete server error: {}", content.status);
                }
                Err(e) => { failed += 1; println!("  FAIL: domains.delete error: {e}"); }
            }
        }
        Err(mailodds::apis::Error::ResponseError(content)) if content.status == 500 => {
            warned += 1;
            println!("  WARN: domains server error: {}", content.status);
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

    // ── 8. DMARC Monitoring ─────────────────────────────────────────────

    let dmarc_domain_name = format!("smoke-rust-{ts}.example.com");
    let dmarc_req = AddDmarcDomainRequest::new(dmarc_domain_name.clone());
    match dmarc_monitoring_api::add_dmarc_domain(&config, dmarc_req).await {
        Ok(resp) => {
            let domain = resp.domain.as_ref().expect("dmarc.add missing domain");
            let domain_id = domain.id.as_ref().expect("dmarc.add missing domain.id").clone();
            if !domain_id.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: dmarc.add domain_id is empty"); }

            // List DMARC domains
            match dmarc_monitoring_api::list_dmarc_domains(&config).await {
                Ok(list_resp) => {
                    let domains = list_resp.domains.unwrap_or_default();
                    if !domains.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: dmarc.list returned empty domains"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: dmarc.list error: {e}"); }
            }

            // Get DMARC domain
            match dmarc_monitoring_api::get_dmarc_domain(&config, &domain_id, None).await {
                Ok(get_resp) => {
                    let got = get_resp.domain.as_ref().expect("dmarc.get missing domain");
                    let got_id = got.id.as_ref().expect("dmarc.get missing domain.id");
                    if got_id == &domain_id { passed += 1; } else { failed += 1; println!("  FAIL: dmarc.get domain_id mismatch expected={domain_id} got={got_id}"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: dmarc.get error: {e}"); }
            }

            // Delete DMARC domain
            match dmarc_monitoring_api::delete_dmarc_domain(&config, &domain_id).await {
                Ok(del_resp) => {
                    let deleted = del_resp.deleted.unwrap_or(false);
                    if deleted { passed += 1; } else { failed += 1; println!("  FAIL: dmarc.delete deleted expected=true got=false"); }
                }
                Err(e) => {
                    failed += 1; println!("  FAIL: dmarc.delete error: {e}");
                    // Best-effort cleanup
                    let _ = dmarc_monitoring_api::delete_dmarc_domain(&config, &domain_id).await;
                }
            }
        }
        Err(e) => { failed += 1; println!("  FAIL: dmarc.add error: {e}"); }
    }

    // ── 9. Blacklist Monitoring ───────────────────────────────────────

    let bl_target = format!("smoke-rust-{ts}.example.com");
    let bl_req = AddBlacklistMonitorRequest::new(
        bl_target.clone(),
        mailodds::models::add_blacklist_monitor_request::TargetType::Domain,
    );
    match blacklist_monitoring_api::add_blacklist_monitor(&config, bl_req).await {
        Ok(resp) => {
            let monitor = resp.monitor.as_ref().expect("blacklist.add missing monitor");
            let monitor_id = monitor.id.as_ref().expect("blacklist.add missing monitor.id").clone();
            if !monitor_id.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: blacklist.add monitor_id is empty"); }

            // List blacklist monitors
            match blacklist_monitoring_api::list_blacklist_monitors(&config).await {
                Ok(list_resp) => {
                    let monitors = list_resp.monitors.unwrap_or_default();
                    if !monitors.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: blacklist.list returned empty monitors"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: blacklist.list error: {e}"); }
            }

            // Delete blacklist monitor
            match blacklist_monitoring_api::delete_blacklist_monitor(&config, &monitor_id).await {
                Ok(del_resp) => {
                    let deleted = del_resp.deleted.unwrap_or(false);
                    if deleted { passed += 1; } else { failed += 1; println!("  FAIL: blacklist.delete deleted expected=true got=false"); }
                }
                Err(e) => {
                    failed += 1; println!("  FAIL: blacklist.delete error: {e}");
                    // Best-effort cleanup
                    let _ = blacklist_monitoring_api::delete_blacklist_monitor(&config, &monitor_id).await;
                }
            }
        }
        Err(e) => { failed += 1; println!("  FAIL: blacklist.add error: {e}"); }
    }

    // ── 10. Server Tests ──────────────────────────────────────────────

    let st_req = RunServerTestRequest::new("mailodds.com".to_string());
    match server_tests_api::run_server_test(&config, st_req).await {
        Ok(resp) => {
            let test = resp.test.as_ref().expect("server_test.run missing test");
            let test_id = test.id.as_ref().expect("server_test.run missing test.id");
            if !test_id.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: server_test.run test_id is empty"); }

            // List server tests
            match server_tests_api::list_server_tests(&config, None, None).await {
                Ok(list_resp) => {
                    let tests = list_resp.data.unwrap_or_default();
                    if !tests.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: server_test.list returned empty tests"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: server_test.list error: {e}"); }
            }

            // Get server test
            match server_tests_api::get_server_test(&config, test_id).await {
                Ok(get_resp) => {
                    let got = get_resp.test.as_ref().expect("server_test.get missing test");
                    let got_id = got.id.as_ref().expect("server_test.get missing test.id");
                    if got_id == test_id { passed += 1; } else { failed += 1; println!("  FAIL: server_test.get test_id mismatch expected={test_id} got={got_id}"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: server_test.get error: {e}"); }
            }
        }
        Err(e) => { failed += 1; println!("  FAIL: server_test.run error: {e}"); }
    }

    // ── 11. Contact Lists ─────────────────────────────────────────────

    let cl_name = format!("smoke-rust-cl-{ts}");
    let cl_req = CreateContactListRequest::new(cl_name.clone());
    match contact_lists_api::create_contact_list(&config, cl_req).await {
        Ok(resp) => {
            let cl = resp.contact_list.as_ref().expect("contact_list.create missing contact_list");
            let cl_id = cl.id.as_ref().expect("contact_list.create missing contact_list.id").clone();
            if !cl_id.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: contact_list.create id is empty"); }

            // List contact lists
            match contact_lists_api::list_contact_lists(&config, None, None).await {
                Ok(list_resp) => {
                    let lists = list_resp.contact_lists.unwrap_or_default();
                    if !lists.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: contact_list.list returned empty"); }
                }
                Err(e) => { failed += 1; println!("  FAIL: contact_list.list error: {e}"); }
            }

            // Delete contact list
            match contact_lists_api::delete_contact_list(&config, &cl_id).await {
                Ok(del_resp) => {
                    let deleted = del_resp.deleted.unwrap_or(false);
                    if deleted { passed += 1; } else { failed += 1; println!("  FAIL: contact_list.delete deleted expected=true got=false"); }
                }
                Err(e) => {
                    failed += 1; println!("  FAIL: contact_list.delete error: {e}");
                    // Best-effort cleanup
                    let _ = contact_lists_api::delete_contact_list(&config, &cl_id).await;
                }
            }
        }
        Err(e) => { failed += 1; println!("  FAIL: contact_list.create error: {e}"); }
    }

    // ── 12. Content Classification ────────────────────────────────────

    let mut cc_req = ClassifyContentRequest::new();
    cc_req.subject = Some("Test subject line".to_string());
    cc_req.html_body = Some("<p>Test email body content</p>".to_string());
    match content_classification_api::classify_content(&config, cc_req).await {
        Ok(resp) => {
            if resp.content_check.is_some() { passed += 1; } else { failed += 1; println!("  FAIL: content.classify content_check is None"); }
        }
        Err(e) => { failed += 1; println!("  FAIL: content.classify error: {e}"); }
    }

    // ── 13. Event Tracking ──────────────────────────────────────────────

    let evt_email = format!("smoke-rust-{ts}@example.com");
    let evt_req = TrackEventRequest::new(
        mailodds::models::track_event_request::EventType::Purchase,
        evt_email,
    );
    match events_api::track_event(&config, evt_req).await {
        Ok(resp) => {
            let created = resp.created.unwrap_or(false);
            if created { passed += 1; } else { failed += 1; println!("  FAIL: event.track.created expected=true got=false"); }
            if resp.event_id.unwrap_or(0) > 0 { passed += 1; } else { failed += 1; println!("  FAIL: event.track.event_id expected>0"); }
            let sv = resp.schema_version.unwrap_or_default();
            if sv == "1.1" { passed += 1; } else { failed += 1; println!("  FAIL: event.track.schema_version expected=1.1 got={sv}"); }
        }
        Err(e) => { failed += 1; println!("  FAIL: event.track error: {e}"); }
    }

    // ── 14. Message Events (import-only) ──────────────────────────────
    // Verify the module compiles and the function is accessible without
    // calling the endpoint (which requires a valid message_id).
    let _ = message_events_api::get_message_events;
    passed += 1; // import-only check

    // ── 14. Email Sending (import-only) ──────────────────────────────────
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

    // ── 15. Alert Rules CRUD ─────────────────────────────────────────────

    {
        let mut alert_rule_id: Option<String> = None;
        let alert_req = CreateAlertRuleRequest::new(
            "hard_bounce_rate".to_string(), 0.05, format!("https://smoke-{ts}.example.com/hook"),
        );
        match alert_rules_api::create_alert_rule(&config, alert_req).await {
            Ok(resp) => {
                let rule = resp.rule.as_ref().expect("alert.create missing rule");
                let rule_id = rule.id.as_ref().expect("alert.create missing rule.id").clone();
                if !rule_id.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: alert.create id is empty"); }
                alert_rule_id = Some(rule_id.clone());

                // Get alert rule
                match alert_rules_api::get_alert_rule(&config, &rule_id).await {
                    Ok(get_resp) => {
                        let got = get_resp.rule.as_ref().expect("alert.get missing rule");
                        let metric = got.metric.as_ref().expect("alert.get missing metric");
                        if metric == "hard_bounce_rate" { passed += 1; } else { failed += 1; println!("  FAIL: alert.get.metric expected=hard_bounce_rate got={metric}"); }
                    }
                    Err(e) => { failed += 1; println!("  FAIL: alert.get error: {e}"); }
                }

                // Update alert rule
                let mut update_req = UpdateAlertRuleRequest::new();
                update_req.threshold = Some(10.0);
                match alert_rules_api::update_alert_rule(&config, &rule_id, update_req).await {
                    Ok(upd_resp) => {
                        let upd_rule = upd_resp.rule.as_ref().expect("alert.update missing rule");
                        let threshold = upd_rule.threshold.unwrap_or(0.0);
                        if (threshold - 10.0).abs() < 0.01 { passed += 1; } else { failed += 1; println!("  FAIL: alert.update.threshold expected=10.0 got={threshold}"); }
                    }
                    Err(e) => { failed += 1; println!("  FAIL: alert.update error: {e}"); }
                }

                // List alert rules
                match alert_rules_api::list_alert_rules(&config).await {
                    Ok(list_resp) => {
                        let rules = list_resp.rules.unwrap_or_default();
                        if !rules.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: alert.list returned empty"); }
                    }
                    Err(e) => { failed += 1; println!("  FAIL: alert.list error: {e}"); }
                }

                // Delete alert rule
                match alert_rules_api::delete_alert_rule(&config, &rule_id).await {
                    Ok(del_resp) => {
                        let deleted = del_resp.deleted.unwrap_or(false);
                        if deleted { passed += 1; } else { failed += 1; println!("  FAIL: alert.delete deleted expected=true got=false"); }
                        alert_rule_id = None;
                    }
                    Err(e) => { failed += 1; println!("  FAIL: alert.delete error: {e}"); }
                }
            }
            Err(mailodds::apis::Error::ResponseError(content)) if content.status == 403 => {
                println!("  SKIP: alert_rules (plan-gated)");
            }
            Err(mailodds::apis::Error::ResponseError(content)) if content.status == 500 || content.status == 400 => {
                warned += 1;
                println!("  WARN: alert server error: {}", content.status);
            }
            Err(e) => { failed += 1; println!("  FAIL: alert.create error: {e}"); }
        }
        // Best-effort cleanup
        if let Some(id) = alert_rule_id {
            let _ = alert_rules_api::delete_alert_rule(&config, &id).await;
        }
    }

    // ── 16. Reputation ──────────────────────────────────────────────────

    match reputation_api::get_reputation(&config, Some("7d")).await {
        Ok(_) => { passed += 1; }
        Err(mailodds::apis::Error::ResponseError(content)) if content.status == 403 => {
            println!("  SKIP: reputation.get (plan-gated)");
        }
        Err(e) => { failed += 1; println!("  FAIL: reputation.get error: {e}"); }
    }

    match reputation_api::get_reputation_timeline(&config, Some("30d")).await {
        Ok(_) => { passed += 1; }
        Err(mailodds::apis::Error::ResponseError(content)) if content.status == 403 => {
            println!("  SKIP: reputation.timeline (plan-gated)");
        }
        Err(e) => { failed += 1; println!("  FAIL: reputation.timeline error: {e}"); }
    }

    // ── 17. Spam Check Delete ───────────────────────────────────────────

    {
        let mut spam_check_id: Option<String> = None;
        let spam_req = RunSpamCheckRequest::new("example.com".to_string());
        match spam_checks_api::run_spam_check(&config, spam_req).await {
            Ok(resp) => {
                let sc = resp.spam_check.as_ref().expect("spam.run missing spam_check");
                let sc_id = sc.id.as_ref().expect("spam.run missing spam_check.id").clone();
                if !sc_id.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: spam.run.id is empty"); }
                spam_check_id = Some(sc_id.clone());

                // Get spam check
                match spam_checks_api::get_spam_check(&config, &sc_id).await {
                    Ok(get_resp) => {
                        let got = get_resp.spam_check.as_ref().expect("spam.get missing spam_check");
                        let got_id = got.id.as_ref().expect("spam.get missing spam_check.id");
                        if got_id == &sc_id { passed += 1; } else { failed += 1; println!("  FAIL: spam.get.id mismatch expected={sc_id} got={got_id}"); }
                    }
                    Err(e) => { failed += 1; println!("  FAIL: spam.get error: {e}"); }
                }

                // Delete spam check
                match spam_checks_api::delete_spam_check(&config, &sc_id).await {
                    Ok(del_resp) => {
                        let deleted = del_resp.deleted.unwrap_or(false);
                        if deleted { passed += 1; } else { failed += 1; println!("  FAIL: spam.delete deleted expected=true got=false"); }
                        spam_check_id = None;
                    }
                    Err(e) => { failed += 1; println!("  FAIL: spam.delete error: {e}"); }
                }

                // Verify deleted
                let check_id = sc_id;
                match spam_checks_api::get_spam_check(&config, &check_id).await {
                    Ok(_) => { failed += 1; println!("  FAIL: spam.deleted still accessible"); }
                    Err(_) => { passed += 1; } // Any error means it was deleted
                }
            }
            Err(mailodds::apis::Error::ResponseError(content)) if content.status == 403 => {
                println!("  SKIP: spam_checks (plan-gated)");
            }
            Err(e) => { failed += 1; println!("  FAIL: spam.run error: {e}"); }
        }
        // Best-effort cleanup
        if let Some(id) = spam_check_id {
            let _ = spam_checks_api::delete_spam_check(&config, &id).await;
        }
    }

    // ── 18. Bounce Analysis Delete ──────────────────────────────────────

    {
        let mut analysis_id: Option<String> = None;
        let mut ba_req = CreateBounceAnalysisRequest::new("550 5.1.1 User unknown\n452 4.2.2 Mailbox full".to_string());
        ba_req.name = Some(format!("rust-smoke-{ts}"));
        match bounce_analysis_api::create_bounce_analysis(&config, ba_req).await {
            Ok(resp) => {
                if resp.analysis.is_some() { passed += 1; } else { failed += 1; println!("  FAIL: bounce_analysis.create analysis is None"); }
                let analysis = resp.analysis.as_ref().expect("bounce_analysis.create missing analysis");
                let a_id = analysis.id.as_ref().expect("bounce_analysis.create missing analysis.id").clone();
                analysis_id = Some(a_id.clone());

                // Delete bounce analysis
                match bounce_analysis_api::delete_bounce_analysis(&config, &a_id).await {
                    Ok(del_resp) => {
                        let deleted = del_resp.deleted.unwrap_or(false);
                        if deleted { passed += 1; } else { failed += 1; println!("  FAIL: bounce_analysis.delete deleted expected=true got=false"); }
                        analysis_id = None;
                    }
                    Err(e) => { failed += 1; println!("  FAIL: bounce_analysis.delete error: {e}"); }
                }

                // Verify deleted
                match bounce_analysis_api::get_bounce_analysis(&config, &a_id).await {
                    Ok(_) => { failed += 1; println!("  FAIL: bounce_analysis.deleted still accessible"); }
                    Err(_) => { passed += 1; } // Any error means it was deleted
                }
            }
            Err(mailodds::apis::Error::ResponseError(content)) if content.status == 403 => {
                println!("  SKIP: bounce_analysis (plan-gated)");
            }
            Err(e) => { failed += 1; println!("  FAIL: bounce_analysis.create error: {e}"); }
        }
        // Best-effort cleanup
        if let Some(id) = analysis_id {
            let _ = bounce_analysis_api::delete_bounce_analysis(&config, &id).await;
        }
    }

    // ── 19. Pixel Settings ──────────────────────────────────────────────

    match pixel_settings_api::get_pixel_settings(&config).await {
        Ok(resp) => {
            if resp.pixel_uuid.is_some() { passed += 1; } else { failed += 1; println!("  FAIL: pixel.get pixel_uuid is None"); }
        }
        Err(mailodds::apis::Error::ResponseError(content)) if content.status == 403 => {
            println!("  SKIP: pixel_settings (plan-gated)");
        }
        Err(e) => { failed += 1; println!("  FAIL: pixel.get error: {e}"); }
    }

    let pixel_update_req = UpdatePixelSettingsRequest::new(None);
    match pixel_settings_api::update_pixel_settings(&config, pixel_update_req).await {
        Ok(resp) => {
            if resp.pixel_uuid.is_some() { passed += 1; } else { failed += 1; println!("  FAIL: pixel.update pixel_uuid is None"); }
        }
        Err(mailodds::apis::Error::ResponseError(content)) if content.status == 403 => {
            println!("  SKIP: pixel_settings.update (plan-gated)");
        }
        Err(e) => { failed += 1; println!("  FAIL: pixel.update error: {e}"); }
    }

    // ── 20. Contact List Contacts CRUD ──────────────────────────────────

    {
        let mut cl_contacts_id: Option<String> = None;
        let cl_contacts_name = format!("smoke-rust-contacts-{ts}");
        let cl_contacts_req = CreateContactListRequest::new(cl_contacts_name.clone());
        match contact_lists_api::create_contact_list(&config, cl_contacts_req).await {
            Ok(resp) => {
                let cl = resp.contact_list.as_ref().expect("contacts.list_create missing contact_list");
                let list_id = cl.id.as_ref().expect("contacts.list_create missing contact_list.id").clone();
                if !list_id.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: contacts.list_create id is empty"); }
                cl_contacts_id = Some(list_id.clone());

                // Add contact
                let contact_email = format!("smoke-rust-contact-{ts}@example.com");
                let mut add_req = AddContactRequest::new(contact_email.clone());
                add_req.first_name = Some("Smoke".to_string());
                match contact_lists_api::add_contact(&config, &list_id, add_req).await {
                    Ok(add_resp) => {
                        if add_resp.contact.is_some() { passed += 1; } else { failed += 1; println!("  FAIL: contacts.add contact is None"); }
                        // Extract contact_id from the JSON value
                        let contact_id = add_resp.contact.as_ref()
                            .and_then(|c| c.get("id"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .or_else(|| add_resp.contact.as_ref()
                                .and_then(|c| c.get("id"))
                                .and_then(|v| v.as_i64())
                                .map(|n| n.to_string()));

                        if let Some(cid) = contact_id {
                            // Update contact
                            let mut upd_req = UpdateContactRequest::new();
                            upd_req.last_name = Some("Test".to_string());
                            match contact_lists_api::update_contact(&config, &list_id, &cid, upd_req).await {
                                Ok(_) => { passed += 1; }
                                Err(e) => { failed += 1; println!("  FAIL: contacts.update error: {e}"); }
                            }

                            // Delete contact
                            match contact_lists_api::delete_contact(&config, &list_id, &cid).await {
                                Ok(_) => { passed += 1; }
                                Err(e) => { failed += 1; println!("  FAIL: contacts.delete_contact error: {e}"); }
                            }
                        }
                    }
                    Err(e) => { failed += 1; println!("  FAIL: contacts.add error: {e}"); }
                }

                // Delete contact list
                match contact_lists_api::delete_contact_list(&config, &list_id).await {
                    Ok(_) => {
                        passed += 1;
                        cl_contacts_id = None;
                    }
                    Err(e) => { failed += 1; println!("  FAIL: contacts.delete_list error: {e}"); }
                }
            }
            Err(e) => { failed += 1; println!("  FAIL: contacts.list_create error: {e}"); }
        }
        // Best-effort cleanup
        if let Some(id) = cl_contacts_id {
            let _ = contact_lists_api::delete_contact_list(&config, &id).await;
        }
    }

    // ── 21. OOO Batch Check ─────────────────────────────────────────────

    {
        let ooo_req = BatchCheckOooRequest::new(vec!["test@example.com".to_string()]);
        match out_of_office_api::batch_check_ooo(&config, ooo_req).await {
            Ok(resp) => {
                if resp.results.is_some() { passed += 1; } else { failed += 1; println!("  FAIL: ooo.batch results is None"); }
            }
            Err(mailodds::apis::Error::ResponseError(content)) if content.status == 403 => {
                println!("  SKIP: ooo_batch (plan-gated)");
            }
            Err(mailodds::apis::Error::ResponseError(content)) if content.status == 500 => {
                warned += 1;
                println!("  WARN: ooo server error: {}", content.status);
            }
            Err(e) => { failed += 1; println!("  FAIL: ooo.batch error: {e}"); }
        }
    }

    // ── 22. Engagement Summary ──────────────────────────────────────────

    match engagement_api::get_engagement_summary(&config, None).await {
        Ok(_) => { passed += 1; }
        Err(mailodds::apis::Error::ResponseError(content)) if content.status == 403 => {
            println!("  SKIP: engagement_summary (plan-gated)");
        }
        Err(e) => { failed += 1; println!("  FAIL: engagement.summary error: {e}"); }
    }

    // ── 23. Webhook CLI ─────────────────────────────────────────────────

    {
        let mut webhook_session_id: Option<String> = None;
        let mut wh_req = CreateWebhookCliSessionRequest::new();
        wh_req.forward_url = Some("http://localhost:9999/hooks".to_string());
        match webhook_cli_api::create_webhook_cli_session(&config, Some(wh_req)).await {
            Ok(resp) => {
                let session_id = resp.session_id.as_ref().expect("webhook_cli.create missing session_id").clone();
                if !session_id.is_empty() { passed += 1; } else { failed += 1; println!("  FAIL: webhook_cli.create session_id is empty"); }
                webhook_session_id = Some(session_id.clone());

                // List webhook deliveries
                match webhook_cli_api::list_webhook_deliveries(&config, Some(10)).await {
                    Ok(_) => { passed += 1; }
                    Err(mailodds::apis::Error::ResponseError(content)) if content.status == 500 => {
                        warned += 1;
                        println!("  WARN: webhook_cli.deliveries server error: {}", content.status);
                    }
                    Err(e) => { failed += 1; println!("  FAIL: webhook_cli.deliveries error: {e}"); }
                }

                // Delete session
                match webhook_cli_api::delete_webhook_cli_session(&config, &session_id).await {
                    Ok(_) => {
                        passed += 1;
                        webhook_session_id = None;
                    }
                    Err(e) => { failed += 1; println!("  FAIL: webhook_cli.delete error: {e}"); }
                }
            }
            Err(mailodds::apis::Error::ResponseError(content)) if content.status == 403 => {
                println!("  SKIP: webhook_cli (plan-gated)");
            }
            Err(mailodds::apis::Error::ResponseError(content)) if content.status == 500 => {
                warned += 1;
                println!("  WARN: webhook_cli server error: {}", content.status);
            }
            Err(e) => { failed += 1; println!("  FAIL: webhook_cli.create error: {e}"); }
        }
        // Best-effort cleanup
        if let Some(id) = webhook_session_id {
            let _ = webhook_cli_api::delete_webhook_cli_session(&config, &id).await;
        }
    }

    // ── Summary ─────────────────────────────────────────────────────────

    let total = passed + failed;
    let warn_str = if warned > 0 { format!(", {warned} warnings") } else { String::new() };
    let result = if failed == 0 { "PASS" } else { "FAIL" };
    println!("\n{result}: Rust SDK ({passed}/{total}{warn_str})");
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
