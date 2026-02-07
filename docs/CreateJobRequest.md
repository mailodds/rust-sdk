# CreateJobRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**emails** | **Vec<String>** | List of emails to validate | 
**dedup** | Option<**bool**> | Remove duplicate emails | [optional][default to false]
**metadata** | Option<**serde_json::Value**> | Custom metadata for the job | [optional]
**webhook_url** | Option<**String**> | URL for completion webhook | [optional]
**idempotency_key** | Option<**String**> | Unique key for idempotent requests | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


