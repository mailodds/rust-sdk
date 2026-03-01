# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** | Job name (from metadata or auto-generated) | 
**status** | **Status** |  (enum: pending, processing, completed, failed, cancelled) | 
**total_count** | **i32** |  | 
**processed_count** | **i32** |  | 
**summary** | Option<[**models::JobSummary**](JobSummary.md)> |  | [optional]
**created_at** | **String** |  | 
**started_at** | Option<**String**> | When processing began. Omitted if not yet started. | [optional]
**completed_at** | Option<**String**> | Omitted if not yet completed. | [optional]
**results_expire_at** | **String** | When job results will be purged | 
**metadata** | Option<**serde_json::Value**> | Custom metadata attached at creation | [optional]
**error_message** | Option<**String**> | Error details. Present only for failed jobs. | [optional]
**request_id** | Option<**String**> | Request ID from the job creation request | [optional]
**artifacts** | Option<[**models::JobArtifacts**](JobArtifacts.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


