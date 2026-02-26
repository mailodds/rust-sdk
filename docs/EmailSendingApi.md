# \EmailSendingApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deliver_batch**](EmailSendingApi.md#deliver_batch) | **POST** /v1/deliver/batch | Send to multiple recipients (max 100)
[**deliver_email**](EmailSendingApi.md#deliver_email) | **POST** /v1/deliver | Send a single email



## deliver_batch

> models::BatchDeliverResponse deliver_batch(batch_deliver_request)
Send to multiple recipients (max 100)

Send a single message to up to 100 recipients. Shares the same message body across all recipients. Each recipient is processed independently.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_deliver_request** | [**BatchDeliverRequest**](BatchDeliverRequest.md) |  | [required] |

### Return type

[**models::BatchDeliverResponse**](BatchDeliverResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deliver_email

> models::DeliverResponse deliver_email(deliver_request)
Send a single email

Send a transactional email through the safety pipeline. Validates recipients, checks domain ownership, and queues for delivery. Requires a verified sending domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deliver_request** | [**DeliverRequest**](DeliverRequest.md) |  | [required] |

### Return type

[**models::DeliverResponse**](DeliverResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

