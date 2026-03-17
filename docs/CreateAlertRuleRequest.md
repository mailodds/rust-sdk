# CreateAlertRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metric** | **String** | Metric to monitor (e.g., bounce_rate, complaint_rate) | 
**threshold** | **f64** | Threshold value to trigger alert | 
**channel** | **String** | Notification channel (e.g., webhook) | 
**window_minutes** | Option<**i32**> | Evaluation window in minutes | [optional][default to 60]
**enabled** | Option<**bool**> |  | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


