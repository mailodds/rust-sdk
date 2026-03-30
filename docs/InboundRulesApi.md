# \InboundRulesApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_inbound_rule**](InboundRulesApi.md#create_inbound_rule) | **POST** /v1/sending-domains/{domain_id}/inbound-rules | Create an inbound rule
[**delete_inbound_rule**](InboundRulesApi.md#delete_inbound_rule) | **DELETE** /v1/sending-domains/{domain_id}/inbound-rules/{rule_id} | Delete an inbound rule
[**dry_run_inbound_rules**](InboundRulesApi.md#dry_run_inbound_rules) | **POST** /v1/sending-domains/{domain_id}/inbound-rules/dry-run | Dry-run inbound rules
[**get_inbound_rule**](InboundRulesApi.md#get_inbound_rule) | **GET** /v1/sending-domains/{domain_id}/inbound-rules/{rule_id} | Get an inbound rule
[**list_inbound_rules**](InboundRulesApi.md#list_inbound_rules) | **GET** /v1/sending-domains/{domain_id}/inbound-rules | List inbound rules
[**update_inbound_rule**](InboundRulesApi.md#update_inbound_rule) | **PUT** /v1/sending-domains/{domain_id}/inbound-rules/{rule_id} | Update an inbound rule



## create_inbound_rule

> create_inbound_rule(domain_id)
Create an inbound rule

Create an inbound routing rule for a sending domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_inbound_rule

> delete_inbound_rule(domain_id, rule_id)
Delete an inbound rule

Delete an inbound routing rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |
**rule_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dry_run_inbound_rules

> dry_run_inbound_rules(domain_id)
Dry-run inbound rules

Evaluate inbound rules against a test message without side effects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inbound_rule

> get_inbound_rule(domain_id, rule_id)
Get an inbound rule

Retrieve a specific inbound routing rule by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |
**rule_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_inbound_rules

> list_inbound_rules(domain_id)
List inbound rules

List all inbound routing rules for a sending domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_inbound_rule

> update_inbound_rule(domain_id, rule_id)
Update an inbound rule

Update an existing inbound routing rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** |  | [required] |
**rule_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

