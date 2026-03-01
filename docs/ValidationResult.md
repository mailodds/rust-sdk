# ValidationResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** |  | 
**status** | **Status** |  (enum: valid, invalid, catch_all, do_not_mail, unknown) | 
**sub_status** | Option<**String**> | Detailed reason. Omitted when none. | [optional]
**action** | **Action** |  (enum: accept, accept_with_caution, reject, retry_later) | 
**domain** | **String** | Email domain | 
**mx_host** | Option<**String**> | Primary MX hostname. Omitted when not resolved. | [optional]
**checks** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Detailed check results (JSONB). Omitted when not available. | [optional]
**suppression** | Option<[**models::ValidationResultSuppression**](ValidationResultSuppression.md)> |  | [optional]
**processed_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


