# \StoreConnectionsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_store**](StoreConnectionsApi.md#create_store) | **POST** /v1/stores | Create a store connection
[**disconnect_store**](StoreConnectionsApi.md#disconnect_store) | **DELETE** /v1/stores/{store_id} | Disconnect a store
[**get_store**](StoreConnectionsApi.md#get_store) | **GET** /v1/stores/{store_id} | Get a store connection
[**get_sync_job_errors**](StoreConnectionsApi.md#get_sync_job_errors) | **GET** /v1/stores/{store_id}/sync-jobs/{job_id}/errors | Get sync job errors
[**list_stores**](StoreConnectionsApi.md#list_stores) | **GET** /v1/stores | List store connections
[**list_sync_jobs**](StoreConnectionsApi.md#list_sync_jobs) | **GET** /v1/stores/{store_id}/sync-jobs | List sync jobs
[**trigger_sync**](StoreConnectionsApi.md#trigger_sync) | **POST** /v1/stores/{store_id}/sync | Trigger product sync
[**update_store**](StoreConnectionsApi.md#update_store) | **PUT** /v1/stores/{store_id} | Update a store connection



## create_store

> models::CreateStore201Response create_store(create_store_request)
Create a store connection

Connect an e-commerce store (WooCommerce, PrestaShop, Shopify, or product feed). After creation, trigger a sync to import products.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_store_request** | [**CreateStoreRequest**](CreateStoreRequest.md) |  | [required] |

### Return type

[**models::CreateStore201Response**](createStore_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_store

> models::DisconnectStore200Response disconnect_store(store_id)
Disconnect a store

Disconnect a store and deactivate its products. Products are retained but marked inactive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | Store connection UUID | [required] |

### Return type

[**models::DisconnectStore200Response**](disconnectStore_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_store

> models::CreateStore201Response get_store(store_id)
Get a store connection

Get details of a specific store connection including sync status and product count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | Store connection UUID | [required] |

### Return type

[**models::CreateStore201Response**](createStore_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sync_job_errors

> models::GetSyncJobErrors200Response get_sync_job_errors(store_id, job_id, page, per_page)
Get sync job errors

Get error details for a sync job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | Store ID | [required] |
**job_id** | **String** | Sync job ID | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 50]

### Return type

[**models::GetSyncJobErrors200Response**](getSyncJobErrors_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_stores

> models::ListStores200Response list_stores(status)
List store connections

List all store connections for the authenticated account. Optionally filter by status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Filter by connection status |  |

### Return type

[**models::ListStores200Response**](listStores_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sync_jobs

> models::ListSyncJobs200Response list_sync_jobs(store_id, page, per_page)
List sync jobs

List sync job history for a store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | Store ID | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ListSyncJobs200Response**](listSyncJobs_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_sync

> models::SyncResponse trigger_sync(store_id, idempotency_key)
Trigger product sync

Trigger a manual product sync for a store. Supports idempotency via the Idempotency-Key header (5 minute TTL).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | Store connection UUID | [required] |
**idempotency_key** | Option<**String**> | Idempotency key to prevent duplicate syncs (5 min TTL) |  |

### Return type

[**models::SyncResponse**](SyncResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_store

> models::CreateStore201Response update_store(store_id, update_store_request)
Update a store connection

Update store settings such as name, sync interval, or credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | Store connection UUID | [required] |
**update_store_request** | [**UpdateStoreRequest**](UpdateStoreRequest.md) |  | [required] |

### Return type

[**models::CreateStore201Response**](createStore_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

