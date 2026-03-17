# \ReputationApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_reputation**](ReputationApi.md#get_reputation) | **GET** /v1/reputation | Get account reputation
[**get_reputation_timeline**](ReputationApi.md#get_reputation_timeline) | **GET** /v1/reputation/timeline | Get reputation timeline



## get_reputation

> models::GetReputation200Response get_reputation(period)
Get account reputation

Get the aggregate reputation score and breakdown for the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**period** | Option<**String**> | Evaluation period |  |[default to 7d]

### Return type

[**models::GetReputation200Response**](getReputation_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reputation_timeline

> models::GetReputationTimeline200Response get_reputation_timeline(period)
Get reputation timeline

Get reputation metrics over time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**period** | Option<**String**> | Timeline period |  |[default to 30d]

### Return type

[**models::GetReputationTimeline200Response**](getReputationTimeline_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

