# \SuppressionListsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_suppression**](SuppressionListsApi.md#add_suppression) | **POST** /v1/suppression | Add suppression entries
[**check_suppression**](SuppressionListsApi.md#check_suppression) | **POST** /v1/suppression/check | Check suppression status
[**get_suppression_stats**](SuppressionListsApi.md#get_suppression_stats) | **GET** /v1/suppression/stats | Get suppression statistics
[**list_suppression**](SuppressionListsApi.md#list_suppression) | **GET** /v1/suppression | List suppression entries
[**remove_suppression**](SuppressionListsApi.md#remove_suppression) | **DELETE** /v1/suppression | Remove suppression entries



## add_suppression

> models::AddSuppressionResponse add_suppression(add_suppression_request)
Add suppression entries

Add emails or domains to the suppression list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_suppression_request** | [**AddSuppressionRequest**](AddSuppressionRequest.md) |  | [required] |

### Return type

[**models::AddSuppressionResponse**](AddSuppressionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_suppression

> models::SuppressionCheckResponse check_suppression(check_suppression_request)
Check suppression status

Check if an email is suppressed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_suppression_request** | [**CheckSuppressionRequest**](CheckSuppressionRequest.md) |  | [required] |

### Return type

[**models::SuppressionCheckResponse**](SuppressionCheckResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_suppression_stats

> models::SuppressionStatsResponse get_suppression_stats()
Get suppression statistics

Get statistics about the suppression list.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SuppressionStatsResponse**](SuppressionStatsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_suppression

> models::SuppressionListResponse list_suppression(page, per_page, r#type, search)
List suppression entries

List all suppression entries for the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 50]
**r#type** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**models::SuppressionListResponse**](SuppressionListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_suppression

> models::RemoveSuppression200Response remove_suppression(remove_suppression_request)
Remove suppression entries

Remove emails or domains from the suppression list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_suppression_request** | [**RemoveSuppressionRequest**](RemoveSuppressionRequest.md) |  | [required] |

### Return type

[**models::RemoveSuppression200Response**](removeSuppression_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

