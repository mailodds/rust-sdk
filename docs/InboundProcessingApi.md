# \InboundProcessingApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**correct_inbound_message**](InboundProcessingApi.md#correct_inbound_message) | **PATCH** /v1/inbound-messages/{message_id}/correction | Correct inbound message classification
[**get_bounce_stats**](InboundProcessingApi.md#get_bounce_stats) | **GET** /v1/bounce-stats | Get bounce statistics
[**get_bounce_stats_summary**](InboundProcessingApi.md#get_bounce_stats_summary) | **GET** /v1/bounce-stats/summary | Get bounce statistics summary
[**get_complaint_assessment**](InboundProcessingApi.md#get_complaint_assessment) | **GET** /v1/complaint-assessment | Get complaint assessment
[**get_inbound_message**](InboundProcessingApi.md#get_inbound_message) | **GET** /v1/inbound-messages/{message_id} | Get inbound message
[**list_inbound_messages**](InboundProcessingApi.md#list_inbound_messages) | **GET** /v1/inbound-messages | List inbound messages



## correct_inbound_message

> models::GetInboundMessage200Response correct_inbound_message(message_id, correct_inbound_message_request)
Correct inbound message classification

Submit a human correction for an inbound message classification. Requires Pro+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | Message ID | [required] |
**correct_inbound_message_request** | [**CorrectInboundMessageRequest**](CorrectInboundMessageRequest.md) |  | [required] |

### Return type

[**models::GetInboundMessage200Response**](getInboundMessage_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bounce_stats

> models::GetBounceStats200Response get_bounce_stats(domain_id, period, group_by)
Get bounce statistics

Get bounce and complaint statistics grouped by time period. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | Option<**String**> | Filter by sending domain ID |  |
**period** | Option<**String**> | Time period |  |[default to 7d]
**group_by** | Option<**String**> | Grouping interval |  |[default to day]

### Return type

[**models::GetBounceStats200Response**](getBounceStats_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bounce_stats_summary

> models::GetBounceStatsSummary200Response get_bounce_stats_summary(domain_id, period)
Get bounce statistics summary

Get aggregated bounce and complaint statistics. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | Option<**String**> | Filter by sending domain ID |  |
**period** | Option<**String**> | Time period |  |[default to 30d]

### Return type

[**models::GetBounceStatsSummary200Response**](getBounceStatsSummary_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_complaint_assessment

> models::GetComplaintAssessment200Response get_complaint_assessment(domain_id, period)
Get complaint assessment

Assess complaint risk based on recent inbound data. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | Option<**String**> | Filter by sending domain ID |  |
**period** | Option<**String**> | Time period |  |[default to 30d]

### Return type

[**models::GetComplaintAssessment200Response**](getComplaintAssessment_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inbound_message

> models::GetInboundMessage200Response get_inbound_message(message_id)
Get inbound message

Get a single inbound message with full body content. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | Message ID | [required] |

### Return type

[**models::GetInboundMessage200Response**](getInboundMessage_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_inbound_messages

> models::ListInboundMessages200Response list_inbound_messages(category, domain_id, since, until, is_read, recipient, search, page, per_page)
List inbound messages

List inbound messages (bounces, complaints, replies, OOO). Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | Option<**String**> | Filter by category |  |
**domain_id** | Option<**String**> | Filter by sending domain ID |  |
**since** | Option<**String**> | Start date (ISO 8601) |  |
**until** | Option<**String**> | End date (ISO 8601) |  |
**is_read** | Option<**bool**> | Filter by read status |  |
**recipient** | Option<**String**> | Filter by original recipient |  |
**search** | Option<**String**> | Search in subject and body |  |
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 50]

### Return type

[**models::ListInboundMessages200Response**](listInboundMessages_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

