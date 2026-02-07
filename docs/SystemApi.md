# \SystemApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_telemetry_summary**](SystemApi.md#get_telemetry_summary) | **GET** /v1/telemetry/summary | Get validation telemetry
[**health_check**](SystemApi.md#health_check) | **GET** /health | Health check



## get_telemetry_summary

> models::TelemetrySummary get_telemetry_summary(window)
Get validation telemetry

Get validation metrics for your account. Useful for building dashboards and monitoring. Supports ETag caching.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**window** | Option<**String**> | Time window for metrics |  |[default to 24h]

### Return type

[**models::TelemetrySummary**](TelemetrySummary.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_check

> models::HealthCheck200Response health_check()
Health check

Check API health status. No authentication required.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HealthCheck200Response**](healthCheck_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

