# ClassifyContent200ResponseContentCheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**Status**> | Overall content status (enum: clean, warning, risky) | [optional]
**flag** | Option<**bool**> | Whether the content is flagged | [optional]
**reason** | Option<**String**> | Human-readable reason for the status | [optional]
**priority** | Option<**i32**> | Priority level (1=lowest, 5=highest) | [optional]
**suggestions** | Option<**Vec<String>**> | Improvement suggestions | [optional]
**duration_ms** | Option<**i32**> | Classification duration in milliseconds | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


