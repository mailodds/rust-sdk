# TelemetrySummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schema_version** | Option<**String**> |  | [optional]
**window** | Option<**Window**> |  (enum: 1h, 24h, 30d) | [optional]
**generated_at** | Option<**String**> |  | [optional]
**timezone** | Option<**String**> |  | [optional]
**totals** | Option<[**models::TelemetrySummaryTotals**](TelemetrySummaryTotals.md)> |  | [optional]
**rates** | Option<[**models::TelemetrySummaryRates**](TelemetrySummaryRates.md)> |  | [optional]
**top_reasons** | Option<[**Vec<models::TelemetrySummaryTopReasonsInner>**](TelemetrySummaryTopReasonsInner.md)> | Top rejection/status reasons | [optional]
**top_domains** | Option<[**Vec<models::TelemetrySummaryTopDomainsInner>**](TelemetrySummaryTopDomainsInner.md)> | Top domains by volume | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


