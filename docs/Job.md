# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**status** | Option<**Status**> |  (enum: pending, processing, completed, failed, cancelled) | [optional]
**total_count** | Option<**i32**> |  | [optional]
**processed_count** | Option<**i32**> |  | [optional]
**progress_percent** | Option<**i32**> |  | [optional]
**summary** | Option<[**models::JobSummary**](JobSummary.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**completed_at** | Option<**String**> |  | [optional]
**metadata** | Option<**serde_json::Value**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


