# GetSenderHealth200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schema_version** | Option<**String**> |  | [optional]
**request_id** | Option<**String**> | Unique request identifier | [optional]
**score** | Option<**i32**> | Overall sender health score (0-100) | [optional]
**grade** | Option<**Grade**> | Letter grade based on score (enum: A+, A, B, C, D, F) | [optional]
**period** | Option<**String**> |  | [optional]
**components** | Option<[**models::GetSenderHealth200ResponseComponents**](GetSenderHealth200ResponseComponents.md)> |  | [optional]
**volume** | Option<[**models::GetSenderHealth200ResponseVolume**](GetSenderHealth200ResponseVolume.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


