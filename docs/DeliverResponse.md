# DeliverResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schema_version** | Option<**String**> |  | [optional]
**request_id** | Option<**String**> | Unique request identifier | [optional]
**message_id** | Option<**String**> | Unique message identifier | [optional]
**status** | Option<**Status**> | Delivery status (enum: queued) | [optional]
**delivery** | Option<[**models::DeliverResponseDelivery**](DeliverResponseDelivery.md)> |  | [optional]
**validation** | Option<**serde_json::Value**> | Pre-send validation results (when validate_first is true) | [optional]
**content_scan** | Option<**serde_json::Value**> | Content scan results | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


