# \ConfigurationSetsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_configuration_set**](ConfigurationSetsApi.md#create_configuration_set) | **POST** /v1/configuration-sets | Create a configuration set
[**delete_configuration_set**](ConfigurationSetsApi.md#delete_configuration_set) | **DELETE** /v1/configuration-sets/{name} | Delete a configuration set
[**get_configuration_set**](ConfigurationSetsApi.md#get_configuration_set) | **GET** /v1/configuration-sets/{name} | Get a configuration set
[**get_configuration_set_metrics**](ConfigurationSetsApi.md#get_configuration_set_metrics) | **GET** /v1/configuration-sets/{name}/metrics | Get configuration set metrics
[**list_configuration_sets**](ConfigurationSetsApi.md#list_configuration_sets) | **GET** /v1/configuration-sets | List configuration sets
[**update_configuration_set**](ConfigurationSetsApi.md#update_configuration_set) | **PUT** /v1/configuration-sets/{name} | Update a configuration set



## create_configuration_set

> create_configuration_set()
Create a configuration set

Create a new configuration set for grouping sending behavior.

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


## delete_configuration_set

> delete_configuration_set(name)
Delete a configuration set

Delete a configuration set by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration_set

> get_configuration_set(name)
Get a configuration set

Retrieve a configuration set by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration_set_metrics

> get_configuration_set_metrics(name)
Get configuration set metrics

Retrieve sending metrics for a configuration set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_configuration_sets

> list_configuration_sets()
List configuration sets

List all configuration sets for the account.

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


## update_configuration_set

> update_configuration_set(name)
Update a configuration set

Update an existing configuration set by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

