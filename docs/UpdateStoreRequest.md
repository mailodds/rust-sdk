# UpdateStoreRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**store_name** | Option<**String**> | Display name for the store | [optional]
**sync_interval_seconds** | Option<**i32**> | Auto-sync interval in seconds (min 1800) | [optional]
**settings** | Option<**serde_json::Value**> | Platform-specific settings | [optional]
**credentials** | Option<**serde_json::Value**> | Updated store credentials (connection is tested before saving) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


