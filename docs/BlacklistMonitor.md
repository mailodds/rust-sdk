# BlacklistMonitor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Monitor UUID | [optional]
**target** | Option<**String**> | IP address or domain being monitored | [optional]
**target_type** | Option<**TargetType**> |  (enum: ip, domain) | [optional]
**status** | Option<**String**> | Current status (clean, listed) | [optional]
**listed_count** | Option<**i32**> | Number of blacklists currently listing this target | [optional]
**last_checked_at** | Option<**String**> |  | [optional]
**latest_check** | Option<[**models::BlacklistMonitorLatestCheck**](BlacklistMonitorLatestCheck.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


