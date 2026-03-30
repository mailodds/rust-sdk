# \ManagedSpfApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_managed_spf**](ManagedSpfApi.md#create_managed_spf) | **POST** /v1/sending-domains/{domain_id}/managed-spf | Create managed SPF record
[**get_managed_spf**](ManagedSpfApi.md#get_managed_spf) | **GET** /v1/sending-domains/{domain_id}/managed-spf | Get managed SPF record
[**refresh_managed_spf**](ManagedSpfApi.md#refresh_managed_spf) | **POST** /v1/sending-domains/{domain_id}/managed-spf/refresh | Refresh managed SPF record
[**update_managed_spf**](ManagedSpfApi.md#update_managed_spf) | **PUT** /v1/sending-domains/{domain_id}/managed-spf | Update managed SPF settings



## create_managed_spf

> create_managed_spf(domain_id)
Create managed SPF record

Create a managed SPF record for a sending domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_managed_spf

> get_managed_spf(domain_id)
Get managed SPF record

Retrieve the managed SPF record for a sending domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_managed_spf

> refresh_managed_spf(domain_id)
Refresh managed SPF record

Re-resolve DNS and refresh the managed SPF record for a sending domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_managed_spf

> update_managed_spf(domain_id)
Update managed SPF settings

Update managed SPF settings such as enabling or disabling for a sending domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

