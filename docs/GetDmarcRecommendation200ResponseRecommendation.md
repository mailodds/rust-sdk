# GetDmarcRecommendation200ResponseRecommendation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_policy** | Option<**String**> | Current DMARC policy (none, quarantine, reject) | [optional]
**recommended_policy** | Option<**String**> | Recommended DMARC policy | [optional]
**confidence** | Option<**f64**> | Confidence level (0-1) | [optional]
**reasons** | Option<**Vec<String>**> | Reasons for the recommendation | [optional]
**ready_to_upgrade** | Option<**bool**> | Whether it is safe to upgrade | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


