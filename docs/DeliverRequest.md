# DeliverRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**to** | [**Vec<models::DeliverRequestToInner>**](DeliverRequestToInner.md) | List of recipient email addresses | 
**from** | **String** | Sender email address (must match sending domain) | 
**subject** | **String** | Email subject line | 
**html** | Option<**String**> | HTML email body | [optional]
**text** | Option<**String**> | Plain text email body | [optional]
**domain_id** | **String** | Sending domain UUID | 
**reply_to** | Option<**String**> | Reply-to address | [optional]
**headers** | Option<**serde_json::Value**> | Extra email headers | [optional]
**tags** | Option<**Vec<String>**> | Tags for categorization | [optional]
**campaign_type** | Option<**CampaignType**> | Campaign type for JSON-LD auto-generation (enum: order_confirmation, shipping_notification, subscription_confirm, review_request, event_invitation, promotional, welcome, password_reset, appointment_reminder, ticket_confirmation) | [optional]
**structured_data** | Option<[**models::DeliverRequestStructuredData**](DeliverRequestStructuredData.md)> |  | [optional]
**options** | Option<[**models::DeliverRequestOptions**](DeliverRequestOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


