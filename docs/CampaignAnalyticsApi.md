# \CampaignAnalyticsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_campaign_ab_results**](CampaignAnalyticsApi.md#get_campaign_ab_results) | **GET** /v1/campaigns/{campaign_id}/ab-results | Get A/B test results
[**get_campaign_attribution**](CampaignAnalyticsApi.md#get_campaign_attribution) | **GET** /v1/campaigns/{campaign_id}/conversions/attribution | Get campaign attribution
[**get_campaign_delivery_confidence**](CampaignAnalyticsApi.md#get_campaign_delivery_confidence) | **GET** /v1/campaigns/{campaign_id}/delivery-confidence | Get pre-send delivery confidence
[**get_campaign_funnel**](CampaignAnalyticsApi.md#get_campaign_funnel) | **GET** /v1/campaigns/{campaign_id}/funnel | Get campaign funnel
[**get_campaign_provider_intelligence**](CampaignAnalyticsApi.md#get_campaign_provider_intelligence) | **GET** /v1/campaigns/{campaign_id}/provider-intelligence | Get provider intelligence



## get_campaign_ab_results

> models::GetCampaignAbResults200Response get_campaign_ab_results(campaign_id)
Get A/B test results

Get per-variant performance metrics for an A/B test campaign including open rate, click rate, and statistical confidence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign UUID | [required] |

### Return type

[**models::GetCampaignAbResults200Response**](getCampaignABResults_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_campaign_attribution

> models::GetCampaignAttribution200Response get_campaign_attribution(campaign_id)
Get campaign attribution

Get first-touch and last-touch attribution comparison for campaign conversions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign UUID | [required] |

### Return type

[**models::GetCampaignAttribution200Response**](getCampaignAttribution_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_campaign_delivery_confidence

> models::GetCampaignDeliveryConfidence200Response get_campaign_delivery_confidence(campaign_id)
Get pre-send delivery confidence

Get a predicted delivery confidence score before sending a campaign. Evaluates list quality, sender reputation, and domain authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign UUID | [required] |

### Return type

[**models::GetCampaignDeliveryConfidence200Response**](getCampaignDeliveryConfidence_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_campaign_funnel

> models::GetCampaignFunnel200Response get_campaign_funnel(campaign_id)
Get campaign funnel

Get the full delivery and engagement funnel for a campaign showing progression from sent through delivered, opened, and clicked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign UUID | [required] |

### Return type

[**models::GetCampaignFunnel200Response**](getCampaignFunnel_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_campaign_provider_intelligence

> models::GetCampaignProviderIntelligence200Response get_campaign_provider_intelligence(campaign_id)
Get provider intelligence

Get per-provider delivery and engagement breakdown for a campaign (e.g. Gmail, Outlook, Yahoo).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign UUID | [required] |

### Return type

[**models::GetCampaignProviderIntelligence200Response**](getCampaignProviderIntelligence_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

