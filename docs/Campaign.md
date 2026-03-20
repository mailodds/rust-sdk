# Campaign

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Campaign UUID | 
**account_id** | Option<**i32**> |  | [optional]
**name** | **String** | Campaign name | 
**status** | **Status** |  (enum: draft, scheduled, sending, sent, cancelled) | 
**domain_id** | **String** | Sending domain UUID | 
**subject** | Option<**String**> |  | [optional]
**from_address** | **String** | Sender email address | 
**reply_to** | Option<**String**> |  | [optional]
**html_body** | Option<**String**> |  | [optional]
**text_body** | Option<**String**> |  | [optional]
**html_body_dark** | Option<**String**> |  | [optional]
**text_body_dark** | Option<**String**> |  | [optional]
**campaign_type** | Option<**String**> |  | [optional]
**auto_detect_schema** | Option<**bool**> |  | [optional]
**promo_annotations** | Option<**serde_json::Value**> |  | [optional]
**throwaway_policy** | Option<**String**> |  | [optional]
**scheduled_at** | Option<**String**> |  | [optional]
**started_at** | Option<**String**> |  | [optional]
**completed_at** | Option<**String**> |  | [optional]
**recipient_count** | Option<**i32**> |  | [optional]
**is_ab_test** | Option<**bool**> |  | [optional]
**winning_variant_id** | Option<**String**> |  | [optional]
**ab_test_config** | Option<**serde_json::Value**> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**stats** | Option<[**models::CampaignStats**](CampaignStats.md)> |  | [optional]
**open_rate** | Option<**f64**> |  | [optional]
**click_rate** | Option<**f64**> |  | [optional]
**created_at** | **String** |  | 
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


