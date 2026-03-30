# \SendingDelegationsApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_delegation**](SendingDelegationsApi.md#create_delegation) | **POST** /v1/sending-domains/{domain_id}/delegations | Create a sending delegation
[**list_delegations**](SendingDelegationsApi.md#list_delegations) | **GET** /v1/sending-domains/{domain_id}/delegations | List sending delegations
[**revoke_delegation**](SendingDelegationsApi.md#revoke_delegation) | **DELETE** /v1/sending-domains/{domain_id}/delegations/{delegation_id} | Revoke a sending delegation



## create_delegation

> create_delegation(domain_id)
Create a sending delegation

Create a sending delegation granting another account permission to send from this domain.

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


## list_delegations

> list_delegations(domain_id)
List sending delegations

List all sending delegations for a domain.

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


## revoke_delegation

> revoke_delegation(domain_id, delegation_id)
Revoke a sending delegation

Revoke a sending delegation, removing the delegate account permission to send.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |
**delegation_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

