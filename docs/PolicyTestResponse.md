# PolicyTestResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schema_version** | Option<**String**> |  | [optional]
**original** | Option<**serde_json::Value**> | Original validation result before policy | [optional]
**modified** | Option<**serde_json::Value**> | Result after policy applied | [optional]
**matched_rule** | Option<**serde_json::Value**> | The rule that matched, or null if none matched | [optional]
**rules_evaluated** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


