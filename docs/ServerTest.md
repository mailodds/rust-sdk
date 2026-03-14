# ServerTest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Test UUID | [optional]
**domain** | Option<**String**> | Tested domain | [optional]
**status** | Option<**String**> | Test status | [optional]
**mx_records** | Option<[**Vec<models::ServerTestMxRecordsInner>**](ServerTestMxRecordsInner.md)> |  | [optional]
**smtp_check** | Option<[**models::ServerTestSmtpCheck**](ServerTestSmtpCheck.md)> |  | [optional]
**dns_checks** | Option<[**models::ServerTestDnsChecks**](ServerTestDnsChecks.md)> |  | [optional]
**score** | Option<**i32**> | Overall server configuration score (0-100) | [optional]
**recommendations** | Option<**Vec<String>**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


