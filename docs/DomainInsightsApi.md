# \DomainInsightsApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_domain_hook_effectiveness**](DomainInsightsApi.md#get_domain_hook_effectiveness) | **GET** /v1/sending-domains/{domain_id}/insights/hook-effectiveness | Get hook effectiveness metrics
[**get_domain_insights_funnel**](DomainInsightsApi.md#get_domain_insights_funnel) | **GET** /v1/sending-domains/{domain_id}/insights/funnel | Get domain engagement funnel
[**get_domain_insights_trends**](DomainInsightsApi.md#get_domain_insights_trends) | **GET** /v1/sending-domains/{domain_id}/insights/trends | Get domain engagement trends



## get_domain_hook_effectiveness

> models::GetDomainHookEffectiveness200Response get_domain_hook_effectiveness(domain_id, days)
Get hook effectiveness metrics

Get webhook delivery effectiveness metrics for a sending domain. Requires Pro+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | Sending domain ID | [required] |
**days** | Option<**i32**> | Lookback period in days |  |[default to 30]

### Return type

[**models::GetDomainHookEffectiveness200Response**](getDomainHookEffectiveness_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_insights_funnel

> models::GetDomainInsightsFunnel200Response get_domain_insights_funnel(domain_id, days)
Get domain engagement funnel

Get engagement funnel for a sending domain (sent > delivered > opened > clicked > converted). Requires Pro+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | Sending domain ID | [required] |
**days** | Option<**i32**> | Lookback period in days |  |[default to 30]

### Return type

[**models::GetDomainInsightsFunnel200Response**](getDomainInsightsFunnel_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_insights_trends

> models::GetDomainInsightsTrends200Response get_domain_insights_trends(domain_id, days)
Get domain engagement trends

Get daily engagement trend data for a sending domain. Requires Pro+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | Sending domain ID | [required] |
**days** | Option<**i32**> | Lookback period in days |  |[default to 30]

### Return type

[**models::GetDomainInsightsTrends200Response**](getDomainInsightsTrends_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

