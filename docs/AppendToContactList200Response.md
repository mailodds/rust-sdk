# AppendToContactList200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schema_version** | Option<**String**> |  | [optional]
**request_id** | Option<**String**> |  | [optional]
**contact_list** | Option<[**models::ContactList**](ContactList.md)> |  | [optional]
**added_count** | Option<**i32**> | Number of new emails added | [optional]
**candidate_count** | Option<**i32**> | Total candidates from jobs | [optional]
**duplicate_count** | Option<**i32**> | Duplicates skipped | [optional]
**breakdown** | Option<**serde_json::Value**> | Per-status breakdown of candidates | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


