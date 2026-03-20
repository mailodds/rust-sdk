# \ServerTestsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_server_test**](ServerTestsApi.md#get_server_test) | **GET** /v1/server-tests/{test_id} | Get server test
[**list_server_tests**](ServerTestsApi.md#list_server_tests) | **GET** /v1/server-tests | List server tests
[**run_server_test**](ServerTestsApi.md#run_server_test) | **POST** /v1/server-tests | Run server test



## get_server_test

> models::RunServerTest201Response get_server_test(test_id)
Get server test

Get the detailed results of a specific server test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_id** | **String** |  | [required] |

### Return type

[**models::RunServerTest201Response**](runServerTest_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_server_tests

> models::ListServerTests200Response list_server_tests(page, per_page)
List server tests

List past server test results with pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ListServerTests200Response**](listServerTests_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_server_test

> models::RunServerTest201Response run_server_test(run_server_test_request)
Run server test

Run an SMTP handshake test and MX configuration audit for a domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_server_test_request** | [**RunServerTestRequest**](RunServerTestRequest.md) |  | [required] |

### Return type

[**models::RunServerTest201Response**](runServerTest_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

