# \SendingDomainsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sending_domain**](SendingDomainsApi.md#create_sending_domain) | **POST** /v1/sending-domains | Add a sending domain
[**delete_sending_domain**](SendingDomainsApi.md#delete_sending_domain) | **DELETE** /v1/sending-domains/{domain_id} | Delete a sending domain
[**get_sending_domain**](SendingDomainsApi.md#get_sending_domain) | **GET** /v1/sending-domains/{domain_id} | Get a sending domain
[**get_sending_domain_identity_score**](SendingDomainsApi.md#get_sending_domain_identity_score) | **GET** /v1/sending-domains/{domain_id}/identity-score | Get domain identity score
[**get_sending_stats**](SendingDomainsApi.md#get_sending_stats) | **GET** /v1/sending-stats | Get sending statistics
[**list_sending_domains**](SendingDomainsApi.md#list_sending_domains) | **GET** /v1/sending-domains | List sending domains
[**verify_sending_domain**](SendingDomainsApi.md#verify_sending_domain) | **POST** /v1/sending-domains/{domain_id}/verify | Verify domain DNS records



## create_sending_domain

> models::CreateSendingDomain201Response create_sending_domain(create_sending_domain_request)
Add a sending domain

Register a new sending domain with NS delegation. After adding, configure DNS records and verify.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_sending_domain_request** | [**CreateSendingDomainRequest**](CreateSendingDomainRequest.md) |  | [required] |

### Return type

[**models::CreateSendingDomain201Response**](createSendingDomain_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sending_domain

> models::DeletePolicyRule200Response delete_sending_domain(domain_id)
Delete a sending domain

Permanently remove a sending domain from the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

[**models::DeletePolicyRule200Response**](deletePolicyRule_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sending_domain

> models::CreateSendingDomain201Response get_sending_domain(domain_id)
Get a sending domain

Get details of a specific sending domain including DNS verification status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

[**models::CreateSendingDomain201Response**](createSendingDomain_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sending_domain_identity_score

> models::GetSendingDomainIdentityScore200Response get_sending_domain_identity_score(domain_id)
Get domain identity score

Get a composite DNS health score for the sending domain, checking DKIM, SPF, DMARC, MX, and return path configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

[**models::GetSendingDomainIdentityScore200Response**](getSendingDomainIdentityScore_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sending_stats

> models::GetSendingStats200Response get_sending_stats(period, domain_id)
Get sending statistics

Get aggregate sending statistics across all domains for the account, including delivery rates, open rates, and click rates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**period** | Option<**String**> | Time period |  |[default to 7d]
**domain_id** | Option<**String**> | Filter by domain |  |

### Return type

[**models::GetSendingStats200Response**](getSendingStats_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sending_domains

> models::ListSendingDomains200Response list_sending_domains()
List sending domains

List all sending domains for the authenticated account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListSendingDomains200Response**](listSendingDomains_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_sending_domain

> models::CreateSendingDomain201Response verify_sending_domain(domain_id)
Verify domain DNS records

Check and verify all DNS records (DKIM, SPF, DMARC, MX, return path) for the sending domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

[**models::CreateSendingDomain201Response**](createSendingDomain_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

