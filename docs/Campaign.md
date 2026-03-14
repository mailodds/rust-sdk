# Campaign

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Campaign UUID | 
**name** | **String** | Campaign name | 
**status** | **Status** |  (enum: draft, scheduled, sending, sent, cancelled) | 
**list_id** | **String** | Target subscriber list UUID | 
**domain_id** | **String** | Sending domain UUID | 
**from_email** | **String** |  | 
**from_name** | Option<**String**> |  | [optional]
**reply_to** | Option<**String**> |  | [optional]
**scheduled_at** | Option<**String**> |  | [optional]
**sent_at** | Option<**String**> |  | [optional]
**cancelled_at** | Option<**String**> |  | [optional]
**variant_count** | Option<**i32**> | Number of A/B variants | [optional]
**stats** | Option<[**models::CampaignStats**](CampaignStats.md)> |  | [optional]
**created_at** | **String** |  | 
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


