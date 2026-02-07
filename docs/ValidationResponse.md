# ValidationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schema_version** | Option<**String**> |  | [optional]
**email** | **String** |  | 
**status** | **Status** | Validation status (enum: valid, invalid, catch_all, do_not_mail, unknown) | 
**sub_status** | Option<**String**> | Detailed status reason | [optional]
**action** | **Action** | Recommended action (enum: accept, accept_with_caution, reject, retry_later) | 
**domain** | Option<**String**> |  | [optional]
**mx_found** | Option<**bool**> |  | [optional]
**smtp_check** | Option<**bool**> |  | [optional]
**disposable** | Option<**bool**> |  | [optional]
**role_account** | Option<**bool**> |  | [optional]
**free_provider** | Option<**bool**> |  | [optional]
**suppression_match** | Option<[**models::ValidationResponseSuppressionMatch**](ValidationResponseSuppressionMatch.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


