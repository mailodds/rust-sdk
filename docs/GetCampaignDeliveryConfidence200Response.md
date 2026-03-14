# GetCampaignDeliveryConfidence200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schema_version** | Option<**String**> |  | [optional]
**request_id** | Option<**String**> | Unique request identifier | [optional]
**campaign_id** | Option<**String**> |  | [optional]
**confidence_score** | Option<**i32**> | Predicted delivery confidence (0-100) | [optional]
**factors** | Option<[**models::GetCampaignDeliveryConfidence200ResponseFactors**](GetCampaignDeliveryConfidence200ResponseFactors.md)> |  | [optional]
**recommendations** | Option<**Vec<String>**> | Actionable recommendations to improve delivery confidence | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


