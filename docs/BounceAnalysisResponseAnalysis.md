# BounceAnalysisResponseAnalysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Analysis UUID | [optional]
**domain_id** | Option<**String**> |  | [optional]
**period** | Option<**String**> |  | [optional]
**status** | Option<**Status**> |  (enum: processing, completed, failed) | [optional]
**total_bounces** | Option<**i32**> |  | [optional]
**hard_bounces** | Option<**i32**> |  | [optional]
**soft_bounces** | Option<**i32**> |  | [optional]
**categories** | Option<[**models::BounceAnalysisResponseAnalysisCategories**](BounceAnalysisResponseAnalysisCategories.md)> |  | [optional]
**top_domains** | Option<[**Vec<models::BounceAnalysisResponseAnalysisTopDomainsInner>**](BounceAnalysisResponseAnalysisTopDomainsInner.md)> | Top bouncing recipient domains | [optional]
**recommendations** | Option<**Vec<String>**> | Actionable recommendations to reduce bounces | [optional]
**created_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


