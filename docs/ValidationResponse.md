# ValidationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schema_version** | **String** |  | 
**email** | **String** |  | 
**status** | **Status** | Validation status (enum: valid, invalid, catch_all, do_not_mail, unknown) | 
**action** | **Action** | Recommended action (enum: accept, accept_with_caution, reject, retry_later) | 
**sub_status** | Option<**SubStatus**> | Detailed status reason. Omitted when none. (enum: format_invalid, mx_missing, mx_timeout, smtp_unreachable, smtp_rejected, disposable, role_account, greylisted, catch_all_detected, suppression_match) | [optional]
**domain** | **String** |  | 
**mx_found** | **bool** | Whether MX records were found for the domain | 
**mx_host** | Option<**String**> | Primary MX hostname. Omitted when MX not resolved. | [optional]
**smtp_check** | Option<**bool**> | Whether SMTP verification passed. Omitted when SMTP not checked. | [optional]
**catch_all** | Option<**bool**> | Whether domain is catch-all. Omitted when SMTP not checked. | [optional]
**disposable** | **bool** | Whether domain is a known disposable email provider | 
**role_account** | **bool** | Whether address is a role account (e.g., info@, admin@) | 
**free_provider** | **bool** | Whether domain is a known free email provider (e.g., gmail.com) | 
**depth** | **Depth** | Validation depth used for this check (enum: standard, enhanced) | 
**processed_at** | **String** | ISO 8601 timestamp of validation | 
**suggested_email** | Option<**String**> | Typo correction suggestion. Omitted when no typo detected. | [optional]
**retry_after_ms** | Option<**i32**> | Suggested retry delay in milliseconds. Present only for retry_later action. | [optional]
**suppression_match** | Option<[**models::ValidationResponseSuppressionMatch**](ValidationResponseSuppressionMatch.md)> |  | [optional]
**policy_applied** | Option<[**models::ValidationResponsePolicyApplied**](ValidationResponsePolicyApplied.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


