# BatchDeliverRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**to** | **Vec<String>** | List of recipient email addresses (max 100) | 
**from** | **String** |  | 
**subject** | **String** |  | 
**html** | Option<**String**> |  | [optional]
**text** | Option<**String**> |  | [optional]
**domain_id** | **String** |  | 
**reply_to** | Option<**String**> |  | [optional]
**headers** | Option<**serde_json::Value**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**campaign_type** | Option<**String**> |  | [optional]
**structured_data** | Option<[**models::BatchDeliverRequestStructuredData**](BatchDeliverRequestStructuredData.md)> |  | [optional]
**options** | Option<**serde_json::Value**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


