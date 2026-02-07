# PolicyRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**r#type** | **Type** | Rule type determines how condition is evaluated (enum: status_override, domain_filter, check_requirement, sub_status_override) | 
**condition** | **serde_json::Value** | Condition depends on rule type. status_override: {status}, domain_filter: {domain_mode, domains}, check_requirement: {check, required}, sub_status_override: {sub_status} | 
**action** | [**models::PolicyRuleAction**](PolicyRuleAction.md) |  | 
**sequence** | Option<**i32**> |  | [optional]
**is_enabled** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


