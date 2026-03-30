# \SenderHealthApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sender_health**](SenderHealthApi.md#get_sender_health) | **GET** /v1/sender-health | Get sender health score
[**get_sender_health_trend**](SenderHealthApi.md#get_sender_health_trend) | **GET** /v1/sender-health/trend | Get sender health trend



## get_sender_health

> models::GetSenderHealth200Response get_sender_health(period)
Get sender health score

Get an aggregate sender health score (0-100) across all sending domains. Factors in delivery rate, bounce rate, complaint rate, and authentication status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**period** | Option<**String**> | Time period for health calculation |  |[default to 30d]

### Return type

[**models::GetSenderHealth200Response**](getSenderHealth_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sender_health_trend

> models::GetSenderHealthTrend200Response get_sender_health_trend(period)
Get sender health trend

Get historical sender health scores over time for trend analysis. Returns daily data points for the requested period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**period** | Option<**String**> | Time period for trend data |  |[default to 30d]

### Return type

[**models::GetSenderHealthTrend200Response**](getSenderHealthTrend_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

