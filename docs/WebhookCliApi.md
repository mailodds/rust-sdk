# \WebhookCliApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook_cli_session**](WebhookCliApi.md#create_webhook_cli_session) | **POST** /v1/webhook-cli/sessions | Create CLI forwarding session
[**delete_webhook_cli_session**](WebhookCliApi.md#delete_webhook_cli_session) | **DELETE** /v1/webhook-cli/sessions/{session_id} | Close CLI session
[**list_webhook_deliveries**](WebhookCliApi.md#list_webhook_deliveries) | **GET** /v1/webhook-cli/deliveries | List recent webhook deliveries
[**replay_webhook_delivery**](WebhookCliApi.md#replay_webhook_delivery) | **POST** /v1/webhook-cli/deliveries/{delivery_id}/replay | Replay webhook delivery



## create_webhook_cli_session

> models::CreateWebhookCliSession201Response create_webhook_cli_session(create_webhook_cli_session_request)
Create CLI forwarding session

Register a new session for receiving webhook events via SSE.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_webhook_cli_session_request** | Option<[**CreateWebhookCliSessionRequest**](CreateWebhookCliSessionRequest.md)> |  |  |

### Return type

[**models::CreateWebhookCliSession201Response**](createWebhookCliSession_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook_cli_session

> models::DeleteWebhookCliSession200Response delete_webhook_cli_session(session_id)
Close CLI session

Close a webhook CLI forwarding session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | Session ID | [required] |

### Return type

[**models::DeleteWebhookCliSession200Response**](deleteWebhookCliSession_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhook_deliveries

> models::ListWebhookDeliveries200Response list_webhook_deliveries(limit)
List recent webhook deliveries

List recent webhook deliveries for replay.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum deliveries to return |  |[default to 50]

### Return type

[**models::ListWebhookDeliveries200Response**](listWebhookDeliveries_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replay_webhook_delivery

> models::ReplayWebhookDelivery200Response replay_webhook_delivery(delivery_id)
Replay webhook delivery

Replay a historical webhook delivery to active CLI sessions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_id** | **i32** | Delivery ID | [required] |

### Return type

[**models::ReplayWebhookDelivery200Response**](replayWebhookDelivery_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

