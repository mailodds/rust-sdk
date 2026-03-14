# \CampaignsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_campaign**](CampaignsApi.md#cancel_campaign) | **POST** /v1/campaigns/{campaign_id}/cancel | Cancel a campaign
[**create_campaign**](CampaignsApi.md#create_campaign) | **POST** /v1/campaigns | Create a campaign
[**create_campaign_variant**](CampaignsApi.md#create_campaign_variant) | **POST** /v1/campaigns/{campaign_id}/variants | Create A/B variant
[**get_campaign**](CampaignsApi.md#get_campaign) | **GET** /v1/campaigns/{campaign_id} | Get campaign with stats
[**list_campaigns**](CampaignsApi.md#list_campaigns) | **GET** /v1/campaigns | List campaigns
[**schedule_campaign**](CampaignsApi.md#schedule_campaign) | **POST** /v1/campaigns/{campaign_id}/schedule | Schedule a campaign
[**send_campaign**](CampaignsApi.md#send_campaign) | **POST** /v1/campaigns/{campaign_id}/send | Send a campaign



## cancel_campaign

> models::CampaignResponse cancel_campaign(campaign_id)
Cancel a campaign

Cancel a scheduled or in-progress campaign. Messages already delivered are not recalled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign UUID | [required] |

### Return type

[**models::CampaignResponse**](CampaignResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_campaign

> models::CampaignResponse create_campaign(create_campaign_request)
Create a campaign

Create a new email campaign. Campaigns target a subscriber list and support A/B variant testing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_campaign_request** | [**CreateCampaignRequest**](CreateCampaignRequest.md) |  | [required] |

### Return type

[**models::CampaignResponse**](CampaignResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_campaign_variant

> models::CreateCampaignVariant201Response create_campaign_variant(campaign_id, create_variant_request)
Create A/B variant

Add an A/B test variant to a campaign. Each variant has its own subject, body, and traffic weight. The campaign must be in draft status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign UUID | [required] |
**create_variant_request** | [**CreateVariantRequest**](CreateVariantRequest.md) |  | [required] |

### Return type

[**models::CreateCampaignVariant201Response**](createCampaignVariant_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_campaign

> models::CampaignResponse get_campaign(campaign_id)
Get campaign with stats

Get a campaign by ID including delivery statistics and engagement metrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign UUID | [required] |

### Return type

[**models::CampaignResponse**](CampaignResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_campaigns

> models::ListCampaigns200Response list_campaigns(page, per_page, status)
List campaigns

List all campaigns for the authenticated account with pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number |  |[default to 1]
**per_page** | Option<**i32**> | Items per page |  |[default to 25]
**status** | Option<**String**> | Filter by campaign status |  |

### Return type

[**models::ListCampaigns200Response**](listCampaigns_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_campaign

> models::CampaignResponse schedule_campaign(campaign_id, schedule_campaign_request)
Schedule a campaign

Schedule a campaign for future delivery. Provide a send_at timestamp in ISO 8601 format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign UUID | [required] |
**schedule_campaign_request** | [**ScheduleCampaignRequest**](ScheduleCampaignRequest.md) |  | [required] |

### Return type

[**models::CampaignResponse**](CampaignResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_campaign

> models::CampaignResponse send_campaign(campaign_id)
Send a campaign

Begin sending a campaign immediately. The campaign must be in draft status with at least one variant and a target list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign UUID | [required] |

### Return type

[**models::CampaignResponse**](CampaignResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

