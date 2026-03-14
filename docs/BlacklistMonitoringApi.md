# \BlacklistMonitoringApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_blacklist_monitor**](BlacklistMonitoringApi.md#add_blacklist_monitor) | **POST** /v1/blacklist-monitors | Add blacklist monitor
[**get_blacklist_history**](BlacklistMonitoringApi.md#get_blacklist_history) | **GET** /v1/blacklist-monitors/{monitor_id}/history | Get blacklist check history
[**list_blacklist_monitors**](BlacklistMonitoringApi.md#list_blacklist_monitors) | **GET** /v1/blacklist-monitors | List blacklist monitors
[**run_blacklist_check**](BlacklistMonitoringApi.md#run_blacklist_check) | **POST** /v1/blacklist-monitors/{monitor_id}/check | Run blacklist check



## add_blacklist_monitor

> models::AddBlacklistMonitor201Response add_blacklist_monitor(add_blacklist_monitor_request)
Add blacklist monitor

Add an IP address or domain to monitor against DNS blacklists. An initial check is run immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_blacklist_monitor_request** | [**AddBlacklistMonitorRequest**](AddBlacklistMonitorRequest.md) |  | [required] |

### Return type

[**models::AddBlacklistMonitor201Response**](addBlacklistMonitor_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blacklist_history

> models::GetBlacklistHistory200Response get_blacklist_history(monitor_id, page, per_page)
Get blacklist check history

Get the listing and delisting timeline for a monitored IP or domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**monitor_id** | **String** | Monitor UUID | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::GetBlacklistHistory200Response**](getBlacklistHistory_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_blacklist_monitors

> models::ListBlacklistMonitors200Response list_blacklist_monitors()
List blacklist monitors

List all blacklist monitors for the authenticated account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListBlacklistMonitors200Response**](listBlacklistMonitors_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_blacklist_check

> models::RunBlacklistCheck200Response run_blacklist_check(monitor_id)
Run blacklist check

Run an on-demand DNSBL check for a monitored IP or domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**monitor_id** | **String** | Monitor UUID | [required] |

### Return type

[**models::RunBlacklistCheck200Response**](runBlacklistCheck_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

