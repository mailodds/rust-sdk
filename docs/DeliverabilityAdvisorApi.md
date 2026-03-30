# \DeliverabilityAdvisorApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dismiss_deliverability_recommendation**](DeliverabilityAdvisorApi.md#dismiss_deliverability_recommendation) | **POST** /v1/deliverability/recommendations/{recommendation_id}/dismiss | Dismiss a deliverability recommendation
[**get_deliverability_recommendations**](DeliverabilityAdvisorApi.md#get_deliverability_recommendations) | **GET** /v1/deliverability/recommendations | Get deliverability recommendations



## dismiss_deliverability_recommendation

> dismiss_deliverability_recommendation(recommendation_id)
Dismiss a deliverability recommendation

Dismiss a deliverability recommendation so it no longer appears.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recommendation_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deliverability_recommendations

> get_deliverability_recommendations()
Get deliverability recommendations

Retrieve actionable deliverability recommendations for the account.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

