# \MessageEventsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_message_events**](MessageEventsApi.md#get_message_events) | **GET** /v1/message-events | Get message events



## get_message_events

> models::GetMessageEvents200Response get_message_events(message_id)
Get message events

Get delivery and engagement events for a specific sent message. Returns events in chronological order with bot detection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | UUID of the sent message | [required] |

### Return type

[**models::GetMessageEvents200Response**](getMessageEvents_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

