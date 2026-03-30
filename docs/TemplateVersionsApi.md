# \TemplateVersionsApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**canary_template_version**](TemplateVersionsApi.md#canary_template_version) | **POST** /v1/campaigns/{campaign_id}/template-versions/{version_id}/canary | Start canary deployment
[**create_template_version**](TemplateVersionsApi.md#create_template_version) | **POST** /v1/campaigns/{campaign_id}/template-versions | Create a template version
[**get_template_version**](TemplateVersionsApi.md#get_template_version) | **GET** /v1/campaigns/{campaign_id}/template-versions/{version_id} | Get a template version
[**list_template_versions**](TemplateVersionsApi.md#list_template_versions) | **GET** /v1/campaigns/{campaign_id}/template-versions | List template versions
[**promote_template_version**](TemplateVersionsApi.md#promote_template_version) | **POST** /v1/campaigns/{campaign_id}/template-versions/{version_id}/promote | Promote a template version
[**rollback_template_version**](TemplateVersionsApi.md#rollback_template_version) | **POST** /v1/campaigns/{campaign_id}/template-versions/rollback | Rollback template version
[**update_template_version**](TemplateVersionsApi.md#update_template_version) | **PUT** /v1/campaigns/{campaign_id}/template-versions/{version_id} | Update a template version



## canary_template_version

> canary_template_version(campaign_id, version_id)
Start canary deployment

Start a canary deployment for a template version with a specified traffic percentage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_template_version

> create_template_version(campaign_id)
Create a template version

Create a new template version for a campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_template_version

> get_template_version(campaign_id, version_id)
Get a template version

Retrieve a specific template version by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_template_versions

> list_template_versions(campaign_id)
List template versions

List all template versions for a campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promote_template_version

> promote_template_version(campaign_id, version_id)
Promote a template version

Promote a template version to active for the campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rollback_template_version

> rollback_template_version(campaign_id)
Rollback template version

Rollback the canary deployment and revert to the previous active version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_template_version

> update_template_version(campaign_id, version_id)
Update a template version

Update an existing template version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

