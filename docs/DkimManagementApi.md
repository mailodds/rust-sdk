# \DkimManagementApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dkim_dns_record**](DkimManagementApi.md#get_dkim_dns_record) | **GET** /v1/sending-domains/{domain_id}/dkim/dns-record | Get DKIM DNS record
[**rotate_dkim**](DkimManagementApi.md#rotate_dkim) | **POST** /v1/sending-domains/{domain_id}/dkim/rotate | Rotate DKIM keys



## get_dkim_dns_record

> get_dkim_dns_record(domain_id)
Get DKIM DNS record

Retrieve the current DKIM DNS record and selector for a sending domain.

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


## rotate_dkim

> rotate_dkim(domain_id)
Rotate DKIM keys

Generate a new DKIM key pair and rotate the selector for a sending domain.

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

