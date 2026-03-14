# RunSpamCheckRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_domain** | **String** | Sending domain to check | 
**links** | Option<**Vec<String>**> | URLs included in the email | [optional]
**subject_preview** | Option<**String**> | Email subject line to analyze | [optional]
**client_scores** | Option<**serde_json::Value**> | Client-side spam scores to include in analysis | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


