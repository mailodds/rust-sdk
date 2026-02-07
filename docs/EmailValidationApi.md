# \EmailValidationApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**validate_email**](EmailValidationApi.md#validate_email) | **POST** /v1/validate | Validate single email



## validate_email

> models::ValidationResponse validate_email(validate_request)
Validate single email

Validate a single email address. Returns detailed validation results including status, sub-status, and recommended action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_request** | [**ValidateRequest**](ValidateRequest.md) |  | [required] |

### Return type

[**models::ValidationResponse**](ValidationResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

