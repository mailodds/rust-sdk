# \EngagementApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_disengaged_contacts**](EngagementApi.md#get_disengaged_contacts) | **GET** /v1/engagement/disengaged | List disengaged contacts
[**get_engagement_score**](EngagementApi.md#get_engagement_score) | **GET** /v1/engagement/score/{email} | Get engagement score
[**get_engagement_summary**](EngagementApi.md#get_engagement_summary) | **GET** /v1/engagement/summary | Get engagement summary
[**suppress_disengaged**](EngagementApi.md#suppress_disengaged) | **POST** /v1/engagement/suppress-disengaged | Suppress disengaged contacts



## get_disengaged_contacts

> models::GetDisengagedContacts200Response get_disengaged_contacts(inactive_days, min_sends, domain_id, page, per_page)
List disengaged contacts

List contacts that have not engaged within the specified period. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inactive_days** | Option<**i32**> | Days of inactivity |  |[default to 90]
**min_sends** | Option<**i32**> | Minimum emails sent to qualify |  |[default to 5]
**domain_id** | Option<**String**> | Filter by sending domain ID |  |
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 100]

### Return type

[**models::GetDisengagedContacts200Response**](getDisengagedContacts_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_engagement_score

> models::GetEngagementScore200Response get_engagement_score(email)
Get engagement score

Get the engagement score for a specific email address. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address | [required] |

### Return type

[**models::GetEngagementScore200Response**](getEngagementScore_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_engagement_summary

> models::GetBounceStatsSummary200Response get_engagement_summary(domain_id)
Get engagement summary

Get aggregate engagement metrics across all contacts. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | Option<**String**> | Filter by sending domain ID |  |

### Return type

[**models::GetBounceStatsSummary200Response**](getBounceStatsSummary_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## suppress_disengaged

> models::SuppressDisengaged200Response suppress_disengaged(suppress_disengaged_request)
Suppress disengaged contacts

Add disengaged contacts to the suppression list. Supports dry_run mode. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**suppress_disengaged_request** | [**SuppressDisengagedRequest**](SuppressDisengagedRequest.md) |  | [required] |

### Return type

[**models::SuppressDisengaged200Response**](suppressDisengaged_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

