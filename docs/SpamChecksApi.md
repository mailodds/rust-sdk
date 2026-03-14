# \SpamChecksApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_spam_check**](SpamChecksApi.md#get_spam_check) | **GET** /v1/spam-checks/{check_id} | Get spam check
[**list_spam_checks**](SpamChecksApi.md#list_spam_checks) | **GET** /v1/spam-checks | List spam checks
[**run_spam_check**](SpamChecksApi.md#run_spam_check) | **POST** /v1/spam-checks | Run spam check



## get_spam_check

> models::RunSpamCheck201Response get_spam_check(check_id)
Get spam check

Get the detailed result of a specific spam check. Currently available to beta accounts only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **String** | Spam check UUID | [required] |

### Return type

[**models::RunSpamCheck201Response**](runSpamCheck_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_spam_checks

> models::ListSpamChecks200Response list_spam_checks(page, per_page)
List spam checks

List past spam check results with pagination. Currently available to beta accounts only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ListSpamChecks200Response**](listSpamChecks_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_spam_check

> models::RunSpamCheck201Response run_spam_check(run_spam_check_request)
Run spam check

Run backend spam checks on email sending parameters. Checks domain reputation, link safety, and subject line quality. Currently available to beta accounts only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_spam_check_request** | [**RunSpamCheckRequest**](RunSpamCheckRequest.md) |  | [required] |

### Return type

[**models::RunSpamCheck201Response**](runSpamCheck_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

