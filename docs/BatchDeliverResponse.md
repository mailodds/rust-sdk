# BatchDeliverResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schema_version** | Option<**String**> |  | [optional]
**request_id** | Option<**String**> | Unique request identifier | [optional]
**total** | Option<**i32**> | Total recipients submitted | [optional]
**accepted** | Option<**i32**> | Number of recipients accepted for delivery | [optional]
**rejected** | Option<[**Vec<models::BatchDeliverResponseRejectedInner>**](BatchDeliverResponseRejectedInner.md)> | Recipients that were rejected (suppressed or failed validation) | [optional]
**status** | Option<**Status**> | Batch status (enum: queued, all_rejected) | [optional]
**delivery** | Option<[**models::BatchDeliverResponseDelivery**](BatchDeliverResponseDelivery.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


