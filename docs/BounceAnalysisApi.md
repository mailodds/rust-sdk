# \BounceAnalysisApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bounce_analysis**](BounceAnalysisApi.md#create_bounce_analysis) | **POST** /v1/bounce-analyses | Analyze bounce logs
[**cross_reference_bounces**](BounceAnalysisApi.md#cross_reference_bounces) | **GET** /v1/bounce-analyses/{analysis_id}/cross-reference | Cross-reference bounces with validation logs
[**delete_bounce_analysis**](BounceAnalysisApi.md#delete_bounce_analysis) | **DELETE** /v1/bounce-analyses/{analysis_id} | Delete bounce analysis
[**get_bounce_analysis**](BounceAnalysisApi.md#get_bounce_analysis) | **GET** /v1/bounce-analyses/{analysis_id} | Get bounce analysis
[**get_bounce_records**](BounceAnalysisApi.md#get_bounce_records) | **GET** /v1/bounce-analyses/{analysis_id}/records | Get bounce records



## create_bounce_analysis

> models::BounceAnalysisResponse create_bounce_analysis(create_bounce_analysis_request)
Analyze bounce logs

Submit bounce log data for analysis. Identifies patterns, categorizes bounce types, and provides remediation recommendations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_bounce_analysis_request** | [**CreateBounceAnalysisRequest**](CreateBounceAnalysisRequest.md) |  | [required] |

### Return type

[**models::BounceAnalysisResponse**](BounceAnalysisResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cross_reference_bounces

> models::CrossReferenceBounces200Response cross_reference_bounces(analysis_id)
Cross-reference bounces with validation logs

Match bounced emails against your validation history to identify emails that were validated as deliverable but later bounced.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analysis_id** | **String** | Bounce analysis UUID | [required] |

### Return type

[**models::CrossReferenceBounces200Response**](crossReferenceBounces_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_bounce_analysis

> models::DeletePolicyRule200Response delete_bounce_analysis(analysis_id)
Delete bounce analysis

Delete a bounce analysis and all associated records.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analysis_id** | **String** | Bounce analysis ID | [required] |

### Return type

[**models::DeletePolicyRule200Response**](deletePolicyRule_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bounce_analysis

> models::BounceAnalysisResponse get_bounce_analysis(analysis_id)
Get bounce analysis

Get the results of a bounce analysis including category breakdown, top offenders, and recommendations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analysis_id** | **String** | Bounce analysis UUID | [required] |

### Return type

[**models::BounceAnalysisResponse**](BounceAnalysisResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bounce_records

> models::GetBounceRecords200Response get_bounce_records(analysis_id, page, per_page, r#type)
Get bounce records

Get individual bounce records from an analysis with pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analysis_id** | **String** | Bounce analysis UUID | [required] |
**page** | Option<**i32**> | Page number |  |[default to 1]
**per_page** | Option<**i32**> | Items per page |  |[default to 50]
**r#type** | Option<**String**> | Filter by bounce type |  |

### Return type

[**models::GetBounceRecords200Response**](getBounceRecords_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

