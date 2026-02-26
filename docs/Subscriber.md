# Subscriber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Subscriber UUID | [optional]
**list_id** | Option<**String**> | List UUID | [optional]
**email** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**status** | Option<**Status**> |  (enum: pending, confirmed, unsubscribed, bounced) | [optional]
**consent_source_ip** | Option<**String**> | IP address of subscription | [optional]
**consent_page_url** | Option<**String**> | Page URL where form was submitted | [optional]
**consent_form_id** | Option<**String**> | Form identifier | [optional]
**consent_timestamp** | Option<**String**> |  | [optional]
**confirmed_at** | Option<**String**> |  | [optional]
**unsubscribed_at** | Option<**String**> |  | [optional]
**validation_result** | Option<**serde_json::Value**> | Email validation result | [optional]
**metadata** | Option<**serde_json::Value**> | Custom metadata | [optional]
**created_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


