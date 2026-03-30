# \EventsApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**track_event**](EventsApi.md#track_event) | **POST** /v1/events/track | Track a commerce event



## track_event

> models::TrackEventResponse track_event(track_event_request)
Track a commerce event

Ingest a commerce event (purchase, cart abandonment, browse, wishlist, review, etc.). Supports idempotency via the idempotency_key field (5 minute Redis TTL + DB unique constraint).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**track_event_request** | [**TrackEventRequest**](TrackEventRequest.md) |  | [required] |

### Return type

[**models::TrackEventResponse**](TrackEventResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

