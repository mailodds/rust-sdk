# MailOdds Rust SDK

Rust client for the MailOdds email deliverability platform. Built for performance-critical validation pipelines, DMARC monitoring, and high-throughput email infrastructure.

[![Crates.io](https://img.shields.io/crates/v/mailodds)](https://crates.io/crates/mailodds)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![API Docs](https://img.shields.io/badge/API-Reference-green)](https://mailodds.com/api-reference)

## Installation

```bash
cargo add mailodds
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
mailodds = "1.0"
```

## Quick Start

```rust
use mailodds::apis::configuration::Configuration;
use mailodds::apis::email_validation_api;
use mailodds::models::ValidateRequest;

#[tokio::main]
async fn main() {
    let mut config = Configuration::default();
    config.bearer_access_token = Some(std::env::var("MAILODDS_API_KEY").unwrap());

    let request = ValidateRequest { email: "user@example.com".to_string(), ..Default::default() };
    let result = email_validation_api::validate_email(&config, request).await.unwrap();

    println!("Status: {:?}, Action: {:?}", result.result.status, result.result.action);
}
```

## MailOdds Platform

This SDK provides access to the full [MailOdds](https://mailodds.com) email deliverability platform:

- [Email Sending API](https://mailodds.com/email-sending-api) - Transactional email with DKIM dual signing and engagement tracking
- [Email Webhooks](https://mailodds.com/email-webhooks) - Real-time event notifications for delivery, opens, clicks, and bounces
- [Email Deliverability Platform](https://mailodds.com/email-deliverability-platform) - Complete deliverability monitoring and optimization
- [DMARC Monitoring](https://mailodds.com/dmarc-monitoring) - Aggregate report parsing with source analysis and policy recommendations
- [Email Spam Checker](https://mailodds.com/email-spam-checker) - Test email content against spam filters before sending
- [Email Validation Policies](https://mailodds.com/email-validation-policies) - Configurable validation rules with preset templates
- [API Reference](https://mailodds.com/api-reference) - Complete endpoint documentation with request/response examples
- [Guide: Email Authentication](https://mailodds.com/guides/email-authentication) - SPF, DKIM, and DMARC setup guide
- [Security](https://mailodds.com/security) - Security practices and data protection

## Features

- **Async with tokio** - All API calls are async, built on tokio for concurrent request handling
- **Zero-copy deserialization** - Efficient JSON parsing with serde for minimal allocation overhead
- **Type-safe enums** - Validation statuses, actions, and sub-statuses modeled as Rust enums
- **reqwest HTTP** - Built on reqwest with connection pooling, timeouts, and TLS support
- **Full API coverage** - Validation, sending, campaigns, DMARC, blacklist monitoring, suppression lists, and more

## Why MailOdds

MailOdds is a complete email deliverability platform built for developers. Every email validated or sent through MailOdds passes through 25+ real-time checks including syntax verification, DNS and MX validation, SMTP mailbox probing, disposable domain detection, and role account identification.

The platform maintains sub-200ms median response times for single validations, 99.9% API uptime, and processes bulk lists of up to 500,000 emails per job. MailOdds supports 11 language SDKs, an MCP server for AI agent integration, a CLI for local development, and a WordPress plugin for no-code deployments.

All email sending uses DKIM dual signing with automated key rotation, and the deliverability monitoring stack covers DMARC aggregate reports, blacklist surveillance across 80+ DNSBLs, and real-time sender health scoring.

## API Reference

Full documentation: [https://mailodds.com/api-reference](https://mailodds.com/api-reference)

All URIs are relative to `https://api.mailodds.com/v1`

<details>
<summary>All Endpoints</summary>

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AgentControlPlaneApi* | [**get_mcp_capabilities**](docs/AgentControlPlaneApi.md#get_mcp_capabilities) | **GET** /v1/mcp/capabilities | Get MCP capabilities
*BlacklistMonitoringApi* | [**add_blacklist_monitor**](docs/BlacklistMonitoringApi.md#add_blacklist_monitor) | **POST** /v1/blacklist-monitors | Add blacklist monitor
*BlacklistMonitoringApi* | [**delete_blacklist_monitor**](docs/BlacklistMonitoringApi.md#delete_blacklist_monitor) | **DELETE** /v1/blacklist-monitors/{monitor_id} | Delete a blacklist monitor
*BlacklistMonitoringApi* | [**get_blacklist_history**](docs/BlacklistMonitoringApi.md#get_blacklist_history) | **GET** /v1/blacklist-monitors/{monitor_id}/history | Get blacklist check history
*BlacklistMonitoringApi* | [**list_blacklist_monitors**](docs/BlacklistMonitoringApi.md#list_blacklist_monitors) | **GET** /v1/blacklist-monitors | List blacklist monitors
*BlacklistMonitoringApi* | [**run_blacklist_check**](docs/BlacklistMonitoringApi.md#run_blacklist_check) | **POST** /v1/blacklist-monitors/{monitor_id}/check | Run blacklist check
*BounceAnalysisApi* | [**create_bounce_analysis**](docs/BounceAnalysisApi.md#create_bounce_analysis) | **POST** /v1/bounce-analyses | Analyze bounce logs
*BounceAnalysisApi* | [**cross_reference_bounces**](docs/BounceAnalysisApi.md#cross_reference_bounces) | **GET** /v1/bounce-analyses/{analysis_id}/cross-reference | Cross-reference bounces with validation logs
*BounceAnalysisApi* | [**get_bounce_analysis**](docs/BounceAnalysisApi.md#get_bounce_analysis) | **GET** /v1/bounce-analyses/{analysis_id} | Get bounce analysis
*BounceAnalysisApi* | [**get_bounce_records**](docs/BounceAnalysisApi.md#get_bounce_records) | **GET** /v1/bounce-analyses/{analysis_id}/records | Get bounce records
*BulkValidationApi* | [**cancel_job**](docs/BulkValidationApi.md#cancel_job) | **POST** /v1/jobs/{job_id}/cancel | Cancel a job
*BulkValidationApi* | [**create_job**](docs/BulkValidationApi.md#create_job) | **POST** /v1/jobs | Create bulk validation job (JSON)
*BulkValidationApi* | [**create_job_from_s3**](docs/BulkValidationApi.md#create_job_from_s3) | **POST** /v1/jobs/upload/s3 | Create job from S3 upload
*BulkValidationApi* | [**create_job_upload**](docs/BulkValidationApi.md#create_job_upload) | **POST** /v1/jobs/upload | Create bulk validation job (file upload)
*BulkValidationApi* | [**delete_job**](docs/BulkValidationApi.md#delete_job) | **DELETE** /v1/jobs/{job_id} | Delete a job
*BulkValidationApi* | [**get_job**](docs/BulkValidationApi.md#get_job) | **GET** /v1/jobs/{job_id} | Get job status
*BulkValidationApi* | [**get_job_results**](docs/BulkValidationApi.md#get_job_results) | **GET** /v1/jobs/{job_id}/results | Get job results
*BulkValidationApi* | [**get_presigned_upload**](docs/BulkValidationApi.md#get_presigned_upload) | **POST** /v1/jobs/upload/presigned | Get S3 presigned upload URL
*BulkValidationApi* | [**list_jobs**](docs/BulkValidationApi.md#list_jobs) | **GET** /v1/jobs | List validation jobs
*CampaignAnalyticsApi* | [**get_campaign_ab_results**](docs/CampaignAnalyticsApi.md#get_campaign_ab_results) | **GET** /v1/campaigns/{campaign_id}/ab-results | Get A/B test results
*CampaignAnalyticsApi* | [**get_campaign_attribution**](docs/CampaignAnalyticsApi.md#get_campaign_attribution) | **GET** /v1/campaigns/{campaign_id}/conversions/attribution | Get campaign attribution
*CampaignAnalyticsApi* | [**get_campaign_delivery_confidence**](docs/CampaignAnalyticsApi.md#get_campaign_delivery_confidence) | **GET** /v1/campaigns/{campaign_id}/delivery-confidence | Get pre-send delivery confidence
*CampaignAnalyticsApi* | [**get_campaign_funnel**](docs/CampaignAnalyticsApi.md#get_campaign_funnel) | **GET** /v1/campaigns/{campaign_id}/funnel | Get campaign funnel
*CampaignAnalyticsApi* | [**get_campaign_provider_intelligence**](docs/CampaignAnalyticsApi.md#get_campaign_provider_intelligence) | **GET** /v1/campaigns/{campaign_id}/provider-intelligence | Get provider intelligence
*CampaignsApi* | [**cancel_campaign**](docs/CampaignsApi.md#cancel_campaign) | **POST** /v1/campaigns/{campaign_id}/cancel | Cancel a campaign
*CampaignsApi* | [**create_campaign**](docs/CampaignsApi.md#create_campaign) | **POST** /v1/campaigns | Create a campaign
*CampaignsApi* | [**create_campaign_variant**](docs/CampaignsApi.md#create_campaign_variant) | **POST** /v1/campaigns/{campaign_id}/variants | Create A/B variant
*CampaignsApi* | [**get_campaign**](docs/CampaignsApi.md#get_campaign) | **GET** /v1/campaigns/{campaign_id} | Get campaign with stats
*CampaignsApi* | [**list_campaigns**](docs/CampaignsApi.md#list_campaigns) | **GET** /v1/campaigns | List campaigns
*CampaignsApi* | [**schedule_campaign**](docs/CampaignsApi.md#schedule_campaign) | **POST** /v1/campaigns/{campaign_id}/schedule | Schedule a campaign
*CampaignsApi* | [**send_campaign**](docs/CampaignsApi.md#send_campaign) | **POST** /v1/campaigns/{campaign_id}/send | Send a campaign
*ContactListsApi* | [**append_to_contact_list**](docs/ContactListsApi.md#append_to_contact_list) | **POST** /v1/contact-lists/{list_id}/append | Append to contact list
*ContactListsApi* | [**create_contact_list**](docs/ContactListsApi.md#create_contact_list) | **POST** /v1/contact-lists | Create contact list
*ContactListsApi* | [**delete_contact_list**](docs/ContactListsApi.md#delete_contact_list) | **DELETE** /v1/contact-lists/{list_id} | Delete a contact list
*ContactListsApi* | [**get_inactive_contacts_report**](docs/ContactListsApi.md#get_inactive_contacts_report) | **GET** /v1/contacts/inactive-report | Get inactive contacts report
*ContactListsApi* | [**list_contact_lists**](docs/ContactListsApi.md#list_contact_lists) | **GET** /v1/contact-lists | List contact lists
*ContactListsApi* | [**query_contact_list**](docs/ContactListsApi.md#query_contact_list) | **POST** /v1/contact-lists/{list_id}/query | Query contact list
*ContentClassificationApi* | [**classify_content**](docs/ContentClassificationApi.md#classify_content) | **POST** /v1/content-check | Classify email content
*DmarcMonitoringApi* | [**add_dmarc_domain**](docs/DmarcMonitoringApi.md#add_dmarc_domain) | **POST** /v1/dmarc-domains | Add DMARC domain
*DmarcMonitoringApi* | [**delete_dmarc_domain**](docs/DmarcMonitoringApi.md#delete_dmarc_domain) | **DELETE** /v1/dmarc-domains/{domain_id} | Delete a DMARC domain
*DmarcMonitoringApi* | [**get_dmarc_domain**](docs/DmarcMonitoringApi.md#get_dmarc_domain) | **GET** /v1/dmarc-domains/{domain_id} | Get DMARC domain
*DmarcMonitoringApi* | [**get_dmarc_recommendation**](docs/DmarcMonitoringApi.md#get_dmarc_recommendation) | **GET** /v1/dmarc-domains/{domain_id}/recommendation | Get DMARC policy recommendation
*DmarcMonitoringApi* | [**get_dmarc_sources**](docs/DmarcMonitoringApi.md#get_dmarc_sources) | **GET** /v1/dmarc-domains/{domain_id}/sources | Get DMARC sending sources
*DmarcMonitoringApi* | [**get_dmarc_trend**](docs/DmarcMonitoringApi.md#get_dmarc_trend) | **GET** /v1/dmarc-domains/{domain_id}/trend | Get DMARC trend
*DmarcMonitoringApi* | [**list_dmarc_domains**](docs/DmarcMonitoringApi.md#list_dmarc_domains) | **GET** /v1/dmarc-domains | List DMARC domains
*DmarcMonitoringApi* | [**verify_dmarc_domain**](docs/DmarcMonitoringApi.md#verify_dmarc_domain) | **POST** /v1/dmarc-domains/{domain_id}/verify | Verify DMARC DNS records
*EmailSendingApi* | [**deliver_batch**](docs/EmailSendingApi.md#deliver_batch) | **POST** /v1/deliver/batch | Send to multiple recipients (max 100)
*EmailSendingApi* | [**deliver_email**](docs/EmailSendingApi.md#deliver_email) | **POST** /v1/deliver | Send a single email
*EmailValidationApi* | [**validate_batch**](docs/EmailValidationApi.md#validate_batch) | **POST** /v1/validate/batch | Validate multiple emails (sync)
*EmailValidationApi* | [**validate_email**](docs/EmailValidationApi.md#validate_email) | **POST** /v1/validate | Validate single email
*EventsApi* | [**track_event**](docs/EventsApi.md#track_event) | **POST** /v1/events/track | Track a commerce event
*MessageEventsApi* | [**get_message_events**](docs/MessageEventsApi.md#get_message_events) | **GET** /v1/message-events | Get message events
*OAuth20Api* | [**create_token**](docs/OAuth20Api.md#create_token) | **POST** /oauth/token | Create token
*OAuth20Api* | [**get_jwks**](docs/OAuth20Api.md#get_jwks) | **GET** /.well-known/jwks.json | Get JSON Web Key Set
*OAuth20Api* | [**introspect_token**](docs/OAuth20Api.md#introspect_token) | **POST** /oauth/introspect | Introspect token
*OAuth20Api* | [**oauth_server_metadata**](docs/OAuth20Api.md#oauth_server_metadata) | **GET** /.well-known/oauth-authorization-server | OAuth server metadata
*OAuth20Api* | [**revoke_token**](docs/OAuth20Api.md#revoke_token) | **POST** /oauth/revoke | Revoke token
*ProductsApi* | [**batch_products**](docs/ProductsApi.md#batch_products) | **POST** /v1/stores/{store_id}/products/batch | Batch push products
*ProductsApi* | [**get_product**](docs/ProductsApi.md#get_product) | **GET** /v1/store-products/{product_id} | Get a product
*ProductsApi* | [**query_products**](docs/ProductsApi.md#query_products) | **GET** /v1/store-products | Query products
*SenderHealthApi* | [**get_sender_health**](docs/SenderHealthApi.md#get_sender_health) | **GET** /v1/sender-health | Get sender health score
*SenderHealthApi* | [**get_sender_health_trend**](docs/SenderHealthApi.md#get_sender_health_trend) | **GET** /v1/sender-health/trend | Get sender health trend
*SendingDomainsApi* | [**create_sending_domain**](docs/SendingDomainsApi.md#create_sending_domain) | **POST** /v1/sending-domains | Add a sending domain
*SendingDomainsApi* | [**delete_sending_domain**](docs/SendingDomainsApi.md#delete_sending_domain) | **DELETE** /v1/sending-domains/{domain_id} | Delete a sending domain
*SendingDomainsApi* | [**get_sending_domain**](docs/SendingDomainsApi.md#get_sending_domain) | **GET** /v1/sending-domains/{domain_id} | Get a sending domain
*SendingDomainsApi* | [**get_sending_domain_identity_score**](docs/SendingDomainsApi.md#get_sending_domain_identity_score) | **GET** /v1/sending-domains/{domain_id}/identity-score | Get domain identity score
*SendingDomainsApi* | [**get_sending_stats**](docs/SendingDomainsApi.md#get_sending_stats) | **GET** /v1/sending-stats | Get sending statistics
*SendingDomainsApi* | [**list_sending_domains**](docs/SendingDomainsApi.md#list_sending_domains) | **GET** /v1/sending-domains | List sending domains
*SendingDomainsApi* | [**verify_sending_domain**](docs/SendingDomainsApi.md#verify_sending_domain) | **POST** /v1/sending-domains/{domain_id}/verify | Verify domain DNS records
*ServerTestsApi* | [**get_server_test**](docs/ServerTestsApi.md#get_server_test) | **GET** /v1/server-tests/{test_id} | Get server test
*ServerTestsApi* | [**list_server_tests**](docs/ServerTestsApi.md#list_server_tests) | **GET** /v1/server-tests | List server tests
*ServerTestsApi* | [**run_server_test**](docs/ServerTestsApi.md#run_server_test) | **POST** /v1/server-tests | Run server test
*SpamChecksApi* | [**get_spam_check**](docs/SpamChecksApi.md#get_spam_check) | **GET** /v1/spam-checks/{check_id} | Get spam check
*SpamChecksApi* | [**list_spam_checks**](docs/SpamChecksApi.md#list_spam_checks) | **GET** /v1/spam-checks | List spam checks
*SpamChecksApi* | [**run_spam_check**](docs/SpamChecksApi.md#run_spam_check) | **POST** /v1/spam-checks | Run spam check
*StoreConnectionsApi* | [**create_store**](docs/StoreConnectionsApi.md#create_store) | **POST** /v1/stores | Create a store connection
*StoreConnectionsApi* | [**disconnect_store**](docs/StoreConnectionsApi.md#disconnect_store) | **DELETE** /v1/stores/{store_id} | Disconnect a store
*StoreConnectionsApi* | [**get_store**](docs/StoreConnectionsApi.md#get_store) | **GET** /v1/stores/{store_id} | Get a store connection
*StoreConnectionsApi* | [**list_stores**](docs/StoreConnectionsApi.md#list_stores) | **GET** /v1/stores | List store connections
*StoreConnectionsApi* | [**trigger_sync**](docs/StoreConnectionsApi.md#trigger_sync) | **POST** /v1/stores/{store_id}/sync | Trigger product sync
*SubscriberListsApi* | [**confirm_subscription**](docs/SubscriberListsApi.md#confirm_subscription) | **GET** /v1/confirm/{token} | Confirm subscription
*SubscriberListsApi* | [**create_list**](docs/SubscriberListsApi.md#create_list) | **POST** /v1/lists | Create a subscriber list
*SubscriberListsApi* | [**delete_list**](docs/SubscriberListsApi.md#delete_list) | **DELETE** /v1/lists/{list_id} | Delete a subscriber list
*SubscriberListsApi* | [**get_list**](docs/SubscriberListsApi.md#get_list) | **GET** /v1/lists/{list_id} | Get a subscriber list
*SubscriberListsApi* | [**get_lists**](docs/SubscriberListsApi.md#get_lists) | **GET** /v1/lists | List subscriber lists
*SubscriberListsApi* | [**get_subscribers**](docs/SubscriberListsApi.md#get_subscribers) | **GET** /v1/lists/{list_id}/subscribers | List subscribers
*SubscriberListsApi* | [**subscribe**](docs/SubscriberListsApi.md#subscribe) | **POST** /v1/subscribe/{list_id} | Subscribe to a list
*SubscriberListsApi* | [**unsubscribe_subscriber**](docs/SubscriberListsApi.md#unsubscribe_subscriber) | **DELETE** /v1/lists/{list_id}/subscribers/{subscriber_id} | Unsubscribe a subscriber
*SuppressionListsApi* | [**add_suppression**](docs/SuppressionListsApi.md#add_suppression) | **POST** /v1/suppression | Add suppression entries
*SuppressionListsApi* | [**check_suppression**](docs/SuppressionListsApi.md#check_suppression) | **POST** /v1/suppression/check | Check suppression status
*SuppressionListsApi* | [**get_suppression_audit_log**](docs/SuppressionListsApi.md#get_suppression_audit_log) | **GET** /v1/suppression/audit | Get suppression audit log
*SuppressionListsApi* | [**get_suppression_stats**](docs/SuppressionListsApi.md#get_suppression_stats) | **GET** /v1/suppression/stats | Get suppression statistics
*SuppressionListsApi* | [**list_suppression**](docs/SuppressionListsApi.md#list_suppression) | **GET** /v1/suppression | List suppression entries
*SuppressionListsApi* | [**remove_suppression**](docs/SuppressionListsApi.md#remove_suppression) | **DELETE** /v1/suppression | Remove suppression entries
*SystemApi* | [**get_telemetry_summary**](docs/SystemApi.md#get_telemetry_summary) | **GET** /v1/telemetry/summary | Get validation telemetry
*SystemApi* | [**health_check**](docs/SystemApi.md#health_check) | **GET** /health | Health check
*ValidationPoliciesApi* | [**add_policy_rule**](docs/ValidationPoliciesApi.md#add_policy_rule) | **POST** /v1/policies/{policy_id}/rules | Add rule to policy
*ValidationPoliciesApi* | [**create_policy**](docs/ValidationPoliciesApi.md#create_policy) | **POST** /v1/policies | Create policy
*ValidationPoliciesApi* | [**create_policy_from_preset**](docs/ValidationPoliciesApi.md#create_policy_from_preset) | **POST** /v1/policies/from-preset | Create policy from preset
*ValidationPoliciesApi* | [**delete_policy**](docs/ValidationPoliciesApi.md#delete_policy) | **DELETE** /v1/policies/{policy_id} | Delete policy
*ValidationPoliciesApi* | [**delete_policy_rule**](docs/ValidationPoliciesApi.md#delete_policy_rule) | **DELETE** /v1/policies/{policy_id}/rules/{rule_id} | Delete rule
*ValidationPoliciesApi* | [**get_policy**](docs/ValidationPoliciesApi.md#get_policy) | **GET** /v1/policies/{policy_id} | Get policy
*ValidationPoliciesApi* | [**get_policy_presets**](docs/ValidationPoliciesApi.md#get_policy_presets) | **GET** /v1/policies/presets | Get policy presets
*ValidationPoliciesApi* | [**list_policies**](docs/ValidationPoliciesApi.md#list_policies) | **GET** /v1/policies | List policies
*ValidationPoliciesApi* | [**test_policy**](docs/ValidationPoliciesApi.md#test_policy) | **POST** /v1/policies/test | Test policy evaluation
*ValidationPoliciesApi* | [**update_policy**](docs/ValidationPoliciesApi.md#update_policy) | **PUT** /v1/policies/{policy_id} | Update policy

</details>

<details>
<summary>All Models</summary>

 - [AddBlacklistMonitor201Response](docs/AddBlacklistMonitor201Response.md)
 - [AddBlacklistMonitorRequest](docs/AddBlacklistMonitorRequest.md)
 - [AddDmarcDomain201Response](docs/AddDmarcDomain201Response.md)
 - [AddDmarcDomainRequest](docs/AddDmarcDomainRequest.md)
 - [AddPolicyRule201Response](docs/AddPolicyRule201Response.md)
 - [AddSuppressionRequest](docs/AddSuppressionRequest.md)
 - [AddSuppressionRequestEntriesInner](docs/AddSuppressionRequestEntriesInner.md)
 - [AddSuppressionResponse](docs/AddSuppressionResponse.md)
 - [AppendToContactList200Response](docs/AppendToContactList200Response.md)
 - [AppendToContactListRequest](docs/AppendToContactListRequest.md)
 - [BatchDeliverRequest](docs/BatchDeliverRequest.md)
 - [BatchDeliverRequestStructuredData](docs/BatchDeliverRequestStructuredData.md)
 - [BatchDeliverResponse](docs/BatchDeliverResponse.md)
 - [BatchDeliverResponseDelivery](docs/BatchDeliverResponseDelivery.md)
 - [BatchDeliverResponseRejectedInner](docs/BatchDeliverResponseRejectedInner.md)
 - [BatchProductsRequest](docs/BatchProductsRequest.md)
 - [BatchProductsRequestProductsInner](docs/BatchProductsRequestProductsInner.md)
 - [BatchProductsResponse](docs/BatchProductsResponse.md)
 - [BatchProductsResponseErrorsInner](docs/BatchProductsResponseErrorsInner.md)
 - [BlacklistMonitor](docs/BlacklistMonitor.md)
 - [BlacklistMonitorLatestCheck](docs/BlacklistMonitorLatestCheck.md)
 - [BounceAnalysisResponse](docs/BounceAnalysisResponse.md)
 - [BounceAnalysisResponseAnalysis](docs/BounceAnalysisResponseAnalysis.md)
 - [BounceAnalysisResponseAnalysisCategories](docs/BounceAnalysisResponseAnalysisCategories.md)
 - [BounceAnalysisResponseAnalysisTopDomainsInner](docs/BounceAnalysisResponseAnalysisTopDomainsInner.md)
 - [Campaign](docs/Campaign.md)
 - [CampaignResponse](docs/CampaignResponse.md)
 - [CampaignStats](docs/CampaignStats.md)
 - [CampaignVariant](docs/CampaignVariant.md)
 - [CheckSuppressionRequest](docs/CheckSuppressionRequest.md)
 - [ClassifyContent200Response](docs/ClassifyContent200Response.md)
 - [ClassifyContent200ResponseContentCheck](docs/ClassifyContent200ResponseContentCheck.md)
 - [ClassifyContentRequest](docs/ClassifyContentRequest.md)
 - [ConfirmSubscription200Response](docs/ConfirmSubscription200Response.md)
 - [ContactList](docs/ContactList.md)
 - [CreateBounceAnalysisRequest](docs/CreateBounceAnalysisRequest.md)
 - [CreateCampaignRequest](docs/CreateCampaignRequest.md)
 - [CreateCampaignVariant201Response](docs/CreateCampaignVariant201Response.md)
 - [CreateContactList201Response](docs/CreateContactList201Response.md)
 - [CreateContactListRequest](docs/CreateContactListRequest.md)
 - [CreateJobFromS3Request](docs/CreateJobFromS3Request.md)
 - [CreateJobRequest](docs/CreateJobRequest.md)
 - [CreateList201Response](docs/CreateList201Response.md)
 - [CreateListRequest](docs/CreateListRequest.md)
 - [CreatePolicyFromPresetRequest](docs/CreatePolicyFromPresetRequest.md)
 - [CreatePolicyRequest](docs/CreatePolicyRequest.md)
 - [CreateSendingDomain201Response](docs/CreateSendingDomain201Response.md)
 - [CreateSendingDomainRequest](docs/CreateSendingDomainRequest.md)
 - [CreateStore201Response](docs/CreateStore201Response.md)
 - [CreateStoreRequest](docs/CreateStoreRequest.md)
 - [CreateToken200Response](docs/CreateToken200Response.md)
 - [CreateVariantRequest](docs/CreateVariantRequest.md)
 - [CrossReferenceBounces200Response](docs/CrossReferenceBounces200Response.md)
 - [CrossReferenceBounces200ResponseCrossReference](docs/CrossReferenceBounces200ResponseCrossReference.md)
 - [CrossReferenceBounces200ResponseCrossReferenceEntriesInner](docs/CrossReferenceBounces200ResponseCrossReferenceEntriesInner.md)
 - [DeleteJob200Response](docs/DeleteJob200Response.md)
 - [DeletePolicy200Response](docs/DeletePolicy200Response.md)
 - [DeletePolicyRule200Response](docs/DeletePolicyRule200Response.md)
 - [DeliverRequest](docs/DeliverRequest.md)
 - [DeliverRequestOptions](docs/DeliverRequestOptions.md)
 - [DeliverRequestStructuredData](docs/DeliverRequestStructuredData.md)
 - [DeliverRequestToInner](docs/DeliverRequestToInner.md)
 - [DeliverResponse](docs/DeliverResponse.md)
 - [DeliverResponseDelivery](docs/DeliverResponseDelivery.md)
 - [DisconnectStore200Response](docs/DisconnectStore200Response.md)
 - [DmarcDomain](docs/DmarcDomain.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [GetBlacklistHistory200Response](docs/GetBlacklistHistory200Response.md)
 - [GetBlacklistHistory200ResponseChecksInner](docs/GetBlacklistHistory200ResponseChecksInner.md)
 - [GetBounceRecords200Response](docs/GetBounceRecords200Response.md)
 - [GetBounceRecords200ResponseRecordsInner](docs/GetBounceRecords200ResponseRecordsInner.md)
 - [GetCampaignAbResults200Response](docs/GetCampaignAbResults200Response.md)
 - [GetCampaignAbResults200ResponseVariantsInner](docs/GetCampaignAbResults200ResponseVariantsInner.md)
 - [GetCampaignAbResults200ResponseWinner](docs/GetCampaignAbResults200ResponseWinner.md)
 - [GetCampaignAttribution200Response](docs/GetCampaignAttribution200Response.md)
 - [GetCampaignAttribution200ResponseAttribution](docs/GetCampaignAttribution200ResponseAttribution.md)
 - [GetCampaignAttribution200ResponseAttributionFirstTouch](docs/GetCampaignAttribution200ResponseAttributionFirstTouch.md)
 - [GetCampaignDeliveryConfidence200Response](docs/GetCampaignDeliveryConfidence200Response.md)
 - [GetCampaignDeliveryConfidence200ResponseFactors](docs/GetCampaignDeliveryConfidence200ResponseFactors.md)
 - [GetCampaignDeliveryConfidence200ResponseFactorsDomainAuth](docs/GetCampaignDeliveryConfidence200ResponseFactorsDomainAuth.md)
 - [GetCampaignDeliveryConfidence200ResponseFactorsListQuality](docs/GetCampaignDeliveryConfidence200ResponseFactorsListQuality.md)
 - [GetCampaignDeliveryConfidence200ResponseFactorsSenderReputation](docs/GetCampaignDeliveryConfidence200ResponseFactorsSenderReputation.md)
 - [GetCampaignFunnel200Response](docs/GetCampaignFunnel200Response.md)
 - [GetCampaignFunnel200ResponseFunnel](docs/GetCampaignFunnel200ResponseFunnel.md)
 - [GetCampaignFunnel200ResponseRates](docs/GetCampaignFunnel200ResponseRates.md)
 - [GetCampaignProviderIntelligence200Response](docs/GetCampaignProviderIntelligence200Response.md)
 - [GetCampaignProviderIntelligence200ResponseProvidersInner](docs/GetCampaignProviderIntelligence200ResponseProvidersInner.md)
 - [GetDmarcDomain200Response](docs/GetDmarcDomain200Response.md)
 - [GetDmarcDomain200ResponseDomain](docs/GetDmarcDomain200ResponseDomain.md)
 - [GetDmarcDomain200ResponseDomainAllOfSummary](docs/GetDmarcDomain200ResponseDomainAllOfSummary.md)
 - [GetDmarcRecommendation200Response](docs/GetDmarcRecommendation200Response.md)
 - [GetDmarcRecommendation200ResponseRecommendation](docs/GetDmarcRecommendation200ResponseRecommendation.md)
 - [GetDmarcSources200Response](docs/GetDmarcSources200Response.md)
 - [GetDmarcSources200ResponseSourcesInner](docs/GetDmarcSources200ResponseSourcesInner.md)
 - [GetDmarcTrend200Response](docs/GetDmarcTrend200Response.md)
 - [GetDmarcTrend200ResponseTrendInner](docs/GetDmarcTrend200ResponseTrendInner.md)
 - [GetInactiveContactsReport200Response](docs/GetInactiveContactsReport200Response.md)
 - [GetInactiveContactsReport200ResponseByListInner](docs/GetInactiveContactsReport200ResponseByListInner.md)
 - [GetLists200Response](docs/GetLists200Response.md)
 - [GetMessageEvents200Response](docs/GetMessageEvents200Response.md)
 - [GetMessageEvents200ResponseClicksInner](docs/GetMessageEvents200ResponseClicksInner.md)
 - [GetMessageEvents200ResponseEventsInner](docs/GetMessageEvents200ResponseEventsInner.md)
 - [GetMessageEvents200ResponseSummary](docs/GetMessageEvents200ResponseSummary.md)
 - [GetPresignedUploadRequest](docs/GetPresignedUploadRequest.md)
 - [GetProduct200Response](docs/GetProduct200Response.md)
 - [GetSenderHealth200Response](docs/GetSenderHealth200Response.md)
 - [GetSenderHealth200ResponseComponents](docs/GetSenderHealth200ResponseComponents.md)
 - [GetSenderHealth200ResponseComponentsDeliveryRate](docs/GetSenderHealth200ResponseComponentsDeliveryRate.md)
 - [GetSenderHealth200ResponseVolume](docs/GetSenderHealth200ResponseVolume.md)
 - [GetSenderHealthTrend200Response](docs/GetSenderHealthTrend200Response.md)
 - [GetSenderHealthTrend200ResponseDataPointsInner](docs/GetSenderHealthTrend200ResponseDataPointsInner.md)
 - [GetSendingDomainIdentityScore200Response](docs/GetSendingDomainIdentityScore200Response.md)
 - [GetSendingStats200Response](docs/GetSendingStats200Response.md)
 - [GetSendingStats200ResponseStats](docs/GetSendingStats200ResponseStats.md)
 - [GetSubscribers200Response](docs/GetSubscribers200Response.md)
 - [HealthCheck200Response](docs/HealthCheck200Response.md)
 - [IdentityScoreCheck](docs/IdentityScoreCheck.md)
 - [IntrospectToken200Response](docs/IntrospectToken200Response.md)
 - [Job](docs/Job.md)
 - [JobArtifacts](docs/JobArtifacts.md)
 - [JobListResponse](docs/JobListResponse.md)
 - [JobResponse](docs/JobResponse.md)
 - [JobSummary](docs/JobSummary.md)
 - [JwksResponse](docs/JwksResponse.md)
 - [JwksResponseKeysInner](docs/JwksResponseKeysInner.md)
 - [ListBlacklistMonitors200Response](docs/ListBlacklistMonitors200Response.md)
 - [ListCampaigns200Response](docs/ListCampaigns200Response.md)
 - [ListContactLists200Response](docs/ListContactLists200Response.md)
 - [ListDmarcDomains200Response](docs/ListDmarcDomains200Response.md)
 - [ListSendingDomains200Response](docs/ListSendingDomains200Response.md)
 - [ListServerTests200Response](docs/ListServerTests200Response.md)
 - [ListSpamChecks200Response](docs/ListSpamChecks200Response.md)
 - [ListStores200Response](docs/ListStores200Response.md)
 - [McpCapabilities](docs/McpCapabilities.md)
 - [McpCapabilitiesPillarsInner](docs/McpCapabilitiesPillarsInner.md)
 - [McpCapabilitiesPillarsInnerToolsInner](docs/McpCapabilitiesPillarsInnerToolsInner.md)
 - [OAuthServerMetadata](docs/OAuthServerMetadata.md)
 - [Pagination](docs/Pagination.md)
 - [Policy](docs/Policy.md)
 - [PolicyListResponse](docs/PolicyListResponse.md)
 - [PolicyListResponseLimits](docs/PolicyListResponseLimits.md)
 - [PolicyPresetsResponse](docs/PolicyPresetsResponse.md)
 - [PolicyPresetsResponsePresetsInner](docs/PolicyPresetsResponsePresetsInner.md)
 - [PolicyResponse](docs/PolicyResponse.md)
 - [PolicyRule](docs/PolicyRule.md)
 - [PolicyRuleAction](docs/PolicyRuleAction.md)
 - [PolicyTestResponse](docs/PolicyTestResponse.md)
 - [PresignedUploadResponse](docs/PresignedUploadResponse.md)
 - [PresignedUploadResponseUpload](docs/PresignedUploadResponseUpload.md)
 - [ProductFacets](docs/ProductFacets.md)
 - [ProductFacetsCategoriesInner](docs/ProductFacetsCategoriesInner.md)
 - [ProductFacetsPriceRangesInner](docs/ProductFacetsPriceRangesInner.md)
 - [ProductFacetsStoresInner](docs/ProductFacetsStoresInner.md)
 - [QueryContactList200Response](docs/QueryContactList200Response.md)
 - [QueryContactList200ResponseEmailsInner](docs/QueryContactList200ResponseEmailsInner.md)
 - [QueryContactListRequest](docs/QueryContactListRequest.md)
 - [QueryContactListRequestFiltersInner](docs/QueryContactListRequestFiltersInner.md)
 - [QueryProducts200Response](docs/QueryProducts200Response.md)
 - [RemoveSuppression200Response](docs/RemoveSuppression200Response.md)
 - [RemoveSuppressionRequest](docs/RemoveSuppressionRequest.md)
 - [ResultsResponse](docs/ResultsResponse.md)
 - [RunBlacklistCheck200Response](docs/RunBlacklistCheck200Response.md)
 - [RunBlacklistCheck200ResponseCheck](docs/RunBlacklistCheck200ResponseCheck.md)
 - [RunServerTest201Response](docs/RunServerTest201Response.md)
 - [RunServerTestRequest](docs/RunServerTestRequest.md)
 - [RunSpamCheck201Response](docs/RunSpamCheck201Response.md)
 - [RunSpamCheckRequest](docs/RunSpamCheckRequest.md)
 - [ScheduleCampaignRequest](docs/ScheduleCampaignRequest.md)
 - [SendingDomain](docs/SendingDomain.md)
 - [SendingDomainDnsRecords](docs/SendingDomainDnsRecords.md)
 - [SendingDomainDnsRecordsNs](docs/SendingDomainDnsRecordsNs.md)
 - [SendingDomainIdentityScore](docs/SendingDomainIdentityScore.md)
 - [SendingDomainIdentityScoreBreakdown](docs/SendingDomainIdentityScoreBreakdown.md)
 - [ServerTest](docs/ServerTest.md)
 - [ServerTestDnsChecks](docs/ServerTestDnsChecks.md)
 - [ServerTestMxRecordsInner](docs/ServerTestMxRecordsInner.md)
 - [ServerTestSmtpCheck](docs/ServerTestSmtpCheck.md)
 - [SpamCheck](docs/SpamCheck.md)
 - [SpamCheckChecks](docs/SpamCheckChecks.md)
 - [StoreConnection](docs/StoreConnection.md)
 - [StoreProduct](docs/StoreProduct.md)
 - [SubscribeRequest](docs/SubscribeRequest.md)
 - [Subscriber](docs/Subscriber.md)
 - [SubscriberList](docs/SubscriberList.md)
 - [SuppressionAuditResponse](docs/SuppressionAuditResponse.md)
 - [SuppressionAuditResponseEntriesInner](docs/SuppressionAuditResponseEntriesInner.md)
 - [SuppressionCheckResponse](docs/SuppressionCheckResponse.md)
 - [SuppressionEntry](docs/SuppressionEntry.md)
 - [SuppressionListResponse](docs/SuppressionListResponse.md)
 - [SuppressionStatsResponse](docs/SuppressionStatsResponse.md)
 - [SuppressionStatsResponseByType](docs/SuppressionStatsResponseByType.md)
 - [SyncResponse](docs/SyncResponse.md)
 - [TelemetrySummary](docs/TelemetrySummary.md)
 - [TelemetrySummaryRates](docs/TelemetrySummaryRates.md)
 - [TelemetrySummaryTopDomainsInner](docs/TelemetrySummaryTopDomainsInner.md)
 - [TelemetrySummaryTopReasonsInner](docs/TelemetrySummaryTopReasonsInner.md)
 - [TelemetrySummaryTotals](docs/TelemetrySummaryTotals.md)
 - [TestPolicyRequest](docs/TestPolicyRequest.md)
 - [TestPolicyRequestTestResult](docs/TestPolicyRequestTestResult.md)
 - [TrackEventRequest](docs/TrackEventRequest.md)
 - [TrackEventResponse](docs/TrackEventResponse.md)
 - [UnsubscribeSubscriber200Response](docs/UnsubscribeSubscriber200Response.md)
 - [UpdatePolicyRequest](docs/UpdatePolicyRequest.md)
 - [ValidateBatch200Response](docs/ValidateBatch200Response.md)
 - [ValidateBatch200ResponseSummary](docs/ValidateBatch200ResponseSummary.md)
 - [ValidateBatchRequest](docs/ValidateBatchRequest.md)
 - [ValidateRequest](docs/ValidateRequest.md)
 - [ValidationResponse](docs/ValidationResponse.md)
 - [ValidationResponsePolicyApplied](docs/ValidationResponsePolicyApplied.md)
 - [ValidationResponseSuppressionMatch](docs/ValidationResponseSuppressionMatch.md)
 - [ValidationResult](docs/ValidationResult.md)
 - [ValidationResultSuppression](docs/ValidationResultSuppression.md)
 - [WebhookEvent](docs/WebhookEvent.md)

</details>

## Other SDKs

| Language | Package | Source |
|----------|---------|--------|
| [Python](https://mailodds.com/sdks) | [PyPI](https://pypi.org/project/mailodds/) | [GitHub](https://github.com/mailodds/python-sdk) |
| [TypeScript](https://mailodds.com/sdks) | [npm](https://www.npmjs.com/package/@mailodds/sdk) | [GitHub](https://github.com/mailodds/typescript-sdk) |
| [PHP](https://mailodds.com/sdks) | [Packagist](https://packagist.org/packages/mailodds/mailodds-php) | [GitHub](https://github.com/mailodds/php-sdk) |
| [Java](https://mailodds.com/sdks) | [GitHub](https://github.com/mailodds/java-sdk) | [GitHub](https://github.com/mailodds/java-sdk) |
| [Go](https://mailodds.com/sdks) | [pkg.go.dev](https://pkg.go.dev/github.com/mailodds/go-sdk) | [GitHub](https://github.com/mailodds/go-sdk) |
| [C# / .NET](https://mailodds.com/sdks) | [GitHub](https://github.com/mailodds/csharp-sdk) | [GitHub](https://github.com/mailodds/csharp-sdk) |
| [Ruby](https://mailodds.com/sdks) | [RubyGems](https://rubygems.org/gems/mailodds) | [GitHub](https://github.com/mailodds/ruby-sdk) |
| [Kotlin](https://mailodds.com/sdks) | [GitHub](https://github.com/mailodds/kotlin-sdk) | [GitHub](https://github.com/mailodds/kotlin-sdk) |
| [Rust](https://mailodds.com/sdks) | [crates.io](https://crates.io/crates/mailodds) | [GitHub](https://github.com/mailodds/rust-sdk) |
| [Swift](https://mailodds.com/sdks) | [GitHub](https://github.com/mailodds/swift-sdk) | [GitHub](https://github.com/mailodds/swift-sdk) |
| [Dart / Flutter](https://mailodds.com/sdks) | [pub.dev](https://pub.dev/packages/mailodds) | [GitHub](https://github.com/mailodds/dart-sdk) |

## Resources

- [MailOdds Dashboard](https://mailodds.com/dashboard)
- [API Reference](https://mailodds.com/api-reference)
- [SDK Overview](https://mailodds.com/sdks)
- [Contact Support](https://mailodds.com/contact)

## License

MIT
