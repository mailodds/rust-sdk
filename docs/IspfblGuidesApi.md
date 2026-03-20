# \IspfblGuidesApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_isp_fbl_guide**](IspfblGuidesApi.md#get_isp_fbl_guide) | **GET** /v1/isp-fbl/guides/{isp_id} | Get ISP FBL guide
[**list_isp_fbl_guides**](IspfblGuidesApi.md#list_isp_fbl_guides) | **GET** /v1/isp-fbl/guides | List ISP FBL guides



## get_isp_fbl_guide

> get_isp_fbl_guide(isp_id)
Get ISP FBL guide

Retrieve a specific ISP feedback loop setup guide.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**isp_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_isp_fbl_guides

> list_isp_fbl_guides()
List ISP FBL guides

List all ISP feedback loop setup guides.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

