# \ContentClassificationApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**classify_content**](ContentClassificationApi.md#classify_content) | **POST** /v1/content-check | Classify email content



## classify_content

> models::ClassifyContent200Response classify_content(classify_content_request)
Classify email content

Run LLM-powered content analysis on email content. Detects spam signals, compliance issues, and content quality. Provide either subject+html_body or raw content text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**classify_content_request** | [**ClassifyContentRequest**](ClassifyContentRequest.md) |  | [required] |

### Return type

[**models::ClassifyContent200Response**](classifyContent_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

