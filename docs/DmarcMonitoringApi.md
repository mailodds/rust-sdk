# \DmarcMonitoringApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_dmarc_domain**](DmarcMonitoringApi.md#add_dmarc_domain) | **POST** /v1/dmarc-domains | Add DMARC domain
[**delete_dmarc_domain**](DmarcMonitoringApi.md#delete_dmarc_domain) | **DELETE** /v1/dmarc-domains/{domain_id} | Delete a DMARC domain
[**get_dmarc_domain**](DmarcMonitoringApi.md#get_dmarc_domain) | **GET** /v1/dmarc-domains/{domain_id} | Get DMARC domain
[**get_dmarc_recommendation**](DmarcMonitoringApi.md#get_dmarc_recommendation) | **GET** /v1/dmarc-domains/{domain_id}/recommendation | Get DMARC policy recommendation
[**get_dmarc_sources**](DmarcMonitoringApi.md#get_dmarc_sources) | **GET** /v1/dmarc-domains/{domain_id}/sources | Get DMARC sending sources
[**get_dmarc_trend**](DmarcMonitoringApi.md#get_dmarc_trend) | **GET** /v1/dmarc-domains/{domain_id}/trend | Get DMARC trend
[**list_dmarc_domains**](DmarcMonitoringApi.md#list_dmarc_domains) | **GET** /v1/dmarc-domains | List DMARC domains
[**verify_dmarc_domain**](DmarcMonitoringApi.md#verify_dmarc_domain) | **POST** /v1/dmarc-domains/{domain_id}/verify | Verify DMARC DNS records



## add_dmarc_domain

> models::AddDmarcDomain201Response add_dmarc_domain(add_dmarc_domain_request)
Add DMARC domain

Add a domain for DMARC monitoring. A unique reporting address is generated for receiving aggregate DMARC reports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_dmarc_domain_request** | [**AddDmarcDomainRequest**](AddDmarcDomainRequest.md) |  | [required] |

### Return type

[**models::AddDmarcDomain201Response**](addDmarcDomain_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dmarc_domain

> models::DeletePolicyRule200Response delete_dmarc_domain(domain_id)
Delete a DMARC domain

Delete a DMARC domain and all its associated reports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | DMARC domain UUID | [required] |

### Return type

[**models::DeletePolicyRule200Response**](deletePolicyRule_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dmarc_domain

> models::GetDmarcDomain200Response get_dmarc_domain(domain_id, days)
Get DMARC domain

Get a single DMARC domain with summary statistics including pass/fail rates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | DMARC domain UUID | [required] |
**days** | Option<**i32**> | Number of days for summary stats |  |[default to 30]

### Return type

[**models::GetDmarcDomain200Response**](getDmarcDomain_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dmarc_recommendation

> models::GetDmarcRecommendation200Response get_dmarc_recommendation(domain_id)
Get DMARC policy recommendation

Get a recommendation for upgrading the domain's DMARC policy based on alignment data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | DMARC domain UUID | [required] |

### Return type

[**models::GetDmarcRecommendation200Response**](getDmarcRecommendation_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dmarc_sources

> models::GetDmarcSources200Response get_dmarc_sources(domain_id, days, page, per_page)
Get DMARC sending sources

Get sending IPs that have sent email for this domain with their DKIM/SPF alignment status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | DMARC domain UUID | [required] |
**days** | Option<**i32**> | Number of days to look back |  |[default to 30]
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::GetDmarcSources200Response**](getDmarcSources_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dmarc_trend

> models::GetDmarcTrend200Response get_dmarc_trend(domain_id, days)
Get DMARC trend

Get daily pass/fail trend data for DMARC authentication over the specified period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | DMARC domain UUID | [required] |
**days** | Option<**i32**> | Number of days of trend data |  |[default to 30]

### Return type

[**models::GetDmarcTrend200Response**](getDmarcTrend_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dmarc_domains

> models::ListDmarcDomains200Response list_dmarc_domains()
List DMARC domains

List all domains being monitored for DMARC compliance.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListDmarcDomains200Response**](listDmarcDomains_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_dmarc_domain

> models::AddDmarcDomain201Response verify_dmarc_domain(domain_id)
Verify DMARC DNS records

Check that the domain has the correct DMARC TXT record pointing to the MailOdds reporting address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | DMARC domain UUID | [required] |

### Return type

[**models::AddDmarcDomain201Response**](addDmarcDomain_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

