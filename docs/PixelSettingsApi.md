# \PixelSettingsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_pixel_settings**](PixelSettingsApi.md#get_pixel_settings) | **GET** /v1/pixel-settings | Get pixel settings
[**update_pixel_settings**](PixelSettingsApi.md#update_pixel_settings) | **PATCH** /v1/pixel-settings | Update pixel settings



## get_pixel_settings

> models::GetPixelSettings200Response get_pixel_settings()
Get pixel settings

Get the web pixel tracking configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetPixelSettings200Response**](getPixelSettings_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pixel_settings

> models::GetPixelSettings200Response update_pixel_settings(update_pixel_settings_request)
Update pixel settings

Update the web pixel subscribe list configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_pixel_settings_request** | [**UpdatePixelSettingsRequest**](UpdatePixelSettingsRequest.md) |  | [required] |

### Return type

[**models::GetPixelSettings200Response**](getPixelSettings_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

