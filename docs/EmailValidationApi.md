# \EmailValidationApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**validate_batch**](EmailValidationApi.md#validate_batch) | **POST** /v1/validate/batch | Validate multiple emails (sync)
[**validate_email**](EmailValidationApi.md#validate_email) | **POST** /v1/validate | Validate single email



## validate_batch

> models::ValidateBatch200Response validate_batch(validate_batch_request)
Validate multiple emails (sync)

Validate up to 100 email addresses synchronously. For larger lists, use the bulk jobs API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_batch_request** | [**ValidateBatchRequest**](ValidateBatchRequest.md) |  | [required] |

### Return type

[**models::ValidateBatch200Response**](validateBatch_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

