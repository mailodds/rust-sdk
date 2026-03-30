# \EventDestinationsApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_event_destination**](EventDestinationsApi.md#create_event_destination) | **POST** /v1/event-destinations | Create an event destination
[**delete_event_destination**](EventDestinationsApi.md#delete_event_destination) | **DELETE** /v1/event-destinations/{destination_id} | Delete an event destination
[**get_event_destination**](EventDestinationsApi.md#get_event_destination) | **GET** /v1/event-destinations/{destination_id} | Get an event destination
[**list_event_destination_templates**](EventDestinationsApi.md#list_event_destination_templates) | **GET** /v1/event-destinations/templates | List event destination templates
[**list_event_destinations**](EventDestinationsApi.md#list_event_destinations) | **GET** /v1/event-destinations | List event destinations
[**list_event_schemas**](EventDestinationsApi.md#list_event_schemas) | **GET** /v1/event-destinations/schemas | List event schemas
[**update_event_destination**](EventDestinationsApi.md#update_event_destination) | **PUT** /v1/event-destinations/{destination_id} | Update an event destination



## create_event_destination

> create_event_destination()
Create an event destination

Create a new event destination for receiving webhook callbacks.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_event_destination

> delete_event_destination(destination_id)
Delete an event destination

Delete an event destination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_destination

> get_event_destination(destination_id)
Get an event destination

Retrieve a single event destination by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_destination_templates

> list_event_destination_templates()
List event destination templates

List pre-built payload templates for event destinations.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_destinations

> list_event_destinations()
List event destinations

List all event destinations for the account.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_schemas

> list_event_schemas()
List event schemas

List JSON schemas for each event type.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_event_destination

> update_event_destination(destination_id)
Update an event destination

Update an existing event destination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

