# \ProductsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_products**](ProductsApi.md#batch_products) | **POST** /v1/stores/{store_id}/products/batch | Batch push products
[**bulk_update_products**](ProductsApi.md#bulk_update_products) | **PATCH** /v1/store-products/bulk | Bulk update products
[**get_product**](ProductsApi.md#get_product) | **GET** /v1/store-products/{product_id} | Get a product
[**query_products**](ProductsApi.md#query_products) | **GET** /v1/store-products | Query products



## batch_products

> models::BatchProductsResponse batch_products(store_id, batch_products_request)
Batch push products

Push up to 100 products to a custom platform store. Creates new products or updates existing ones matched by external_id. Only available for stores with platform=custom.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | Store connection UUID | [required] |
**batch_products_request** | [**BatchProductsRequest**](BatchProductsRequest.md) |  | [required] |

### Return type

[**models::BatchProductsResponse**](BatchProductsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_products

> models::BulkUpdateProducts200Response bulk_update_products(bulk_update_products_request)
Bulk update products

Bulk update product visibility. Maximum 500 products per request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_update_products_request** | [**BulkUpdateProductsRequest**](BulkUpdateProductsRequest.md) |  | [required] |

### Return type

[**models::BulkUpdateProducts200Response**](bulkUpdateProducts_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product

> models::GetProduct200Response get_product(product_id)
Get a product

Get detailed information about a specific product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** | Product UUID | [required] |

### Return type

[**models::GetProduct200Response**](getProduct_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_products

> models::QueryProducts200Response query_products(store_id, category, stock_status, on_sale, search, facets, group_by_sku, page, per_page)
Query products

Search and filter products across all connected stores. Supports faceted search and cross-store SKU deduplication for unified inventory views.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | Option<**String**> | Filter by store connection UUID |  |
**category** | Option<**String**> | Filter by category name |  |
**stock_status** | Option<**String**> | Filter by stock status |  |
**on_sale** | Option<**bool**> | Filter to products currently on sale |  |
**search** | Option<**String**> | Search by title or SKU |  |
**facets** | Option<**bool**> | Include facet aggregations (categories, price ranges, stores) |  |[default to false]
**group_by_sku** | Option<**bool**> | Merge products with same SKU across stores into unified entries |  |[default to false]
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::QueryProducts200Response**](queryProducts_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

