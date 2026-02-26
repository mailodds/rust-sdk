# \SubscriberListsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirm_subscription**](SubscriberListsApi.md#confirm_subscription) | **GET** /v1/confirm/{token} | Confirm subscription
[**create_list**](SubscriberListsApi.md#create_list) | **POST** /v1/lists | Create a subscriber list
[**delete_list**](SubscriberListsApi.md#delete_list) | **DELETE** /v1/lists/{list_id} | Delete a subscriber list
[**get_list**](SubscriberListsApi.md#get_list) | **GET** /v1/lists/{list_id} | Get a subscriber list
[**get_lists**](SubscriberListsApi.md#get_lists) | **GET** /v1/lists | List subscriber lists
[**get_subscribers**](SubscriberListsApi.md#get_subscribers) | **GET** /v1/lists/{list_id}/subscribers | List subscribers
[**subscribe**](SubscriberListsApi.md#subscribe) | **POST** /v1/subscribe/{list_id} | Subscribe to a list
[**unsubscribe_subscriber**](SubscriberListsApi.md#unsubscribe_subscriber) | **DELETE** /v1/lists/{list_id}/subscribers/{subscriber_id} | Unsubscribe a subscriber



## confirm_subscription

> models::ConfirmSubscription200Response confirm_subscription(token)
Confirm subscription

Confirm a pending subscription via the token sent in the confirmation email. No authentication required. Redirects to the list's configured redirect URL if set, otherwise returns JSON. Tokens expire after 72 hours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Confirmation token from email | [required] |

### Return type

[**models::ConfirmSubscription200Response**](confirmSubscription_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_list

> models::CreateList201Response create_list(create_list_request)
Create a subscriber list

Create a new subscriber list. Use lists to organize subscribers and manage double opt-in confirmation flows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_list_request** | [**CreateListRequest**](CreateListRequest.md) |  | [required] |

### Return type

[**models::CreateList201Response**](createList_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_list

> models::DeletePolicyRule200Response delete_list(list_id)
Delete a subscriber list

Soft-delete a subscriber list. Existing subscribers are retained but the list is no longer usable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** | List UUID | [required] |

### Return type

[**models::DeletePolicyRule200Response**](deletePolicyRule_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list

> models::CreateList201Response get_list(list_id)
Get a subscriber list

Get details of a specific subscriber list including subscriber and confirmed counts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** | List UUID | [required] |

### Return type

[**models::CreateList201Response**](createList_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lists

> models::GetLists200Response get_lists(page, per_page)
List subscriber lists

List all subscriber lists for the authenticated account with pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number |  |[default to 1]
**per_page** | Option<**i32**> | Items per page |  |[default to 25]

### Return type

[**models::GetLists200Response**](getLists_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscribers

> models::GetSubscribers200Response get_subscribers(list_id, page, per_page, status)
List subscribers

List paginated subscribers for a specific list. Optionally filter by status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** | List UUID | [required] |
**page** | Option<**i32**> | Page number |  |[default to 1]
**per_page** | Option<**i32**> | Items per page |  |[default to 50]
**status** | Option<**String**> | Filter by subscriber status |  |

### Return type

[**models::GetSubscribers200Response**](getSubscribers_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe

> models::UnsubscribeSubscriber200Response subscribe(list_id, subscribe_request)
Subscribe to a list

Add a subscriber to a list and initiate the double opt-in confirmation flow. The subscriber receives a confirmation email and must click the link to confirm. Rate limited to 10 requests/min per IP and 1000/hour per account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** | List UUID | [required] |
**subscribe_request** | [**SubscribeRequest**](SubscribeRequest.md) |  | [required] |

### Return type

[**models::UnsubscribeSubscriber200Response**](unsubscribeSubscriber_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe_subscriber

> models::UnsubscribeSubscriber200Response unsubscribe_subscriber(list_id, subscriber_id)
Unsubscribe a subscriber

Set a subscriber's status to unsubscribed. The consent record is retained for compliance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** | List UUID | [required] |
**subscriber_id** | **String** | Subscriber UUID | [required] |

### Return type

[**models::UnsubscribeSubscriber200Response**](unsubscribeSubscriber_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

