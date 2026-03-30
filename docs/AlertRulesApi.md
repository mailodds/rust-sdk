# \AlertRulesApi

All URIs are relative to *https://api.mailodds.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alert_rule**](AlertRulesApi.md#create_alert_rule) | **POST** /v1/alert-rules | Create alert rule
[**delete_alert_rule**](AlertRulesApi.md#delete_alert_rule) | **DELETE** /v1/alert-rules/{rule_id} | Delete alert rule
[**get_alert_rule**](AlertRulesApi.md#get_alert_rule) | **GET** /v1/alert-rules/{rule_id} | Get alert rule
[**list_alert_rules**](AlertRulesApi.md#list_alert_rules) | **GET** /v1/alert-rules | List alert rules
[**update_alert_rule**](AlertRulesApi.md#update_alert_rule) | **PUT** /v1/alert-rules/{rule_id} | Update alert rule



## create_alert_rule

> models::CreateAlertRule201Response create_alert_rule(create_alert_rule_request)
Create alert rule

Create a new metric threshold alert rule. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_alert_rule_request** | [**CreateAlertRuleRequest**](CreateAlertRuleRequest.md) |  | [required] |

### Return type

[**models::CreateAlertRule201Response**](createAlertRule_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alert_rule

> models::DeletePolicyRule200Response delete_alert_rule(rule_id)
Delete alert rule

Delete an alert rule. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |

### Return type

[**models::DeletePolicyRule200Response**](deletePolicyRule_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alert_rule

> models::CreateAlertRule201Response get_alert_rule(rule_id)
Get alert rule

Get a single alert rule by ID. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |

### Return type

[**models::CreateAlertRule201Response**](createAlertRule_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alert_rules

> models::ListAlertRules200Response list_alert_rules()
List alert rules

List all configured alert rules. Requires Growth+ plan.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListAlertRules200Response**](listAlertRules_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_alert_rule

> models::CreateAlertRule201Response update_alert_rule(rule_id, update_alert_rule_request)
Update alert rule

Update an existing alert rule. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |
**update_alert_rule_request** | [**UpdateAlertRuleRequest**](UpdateAlertRuleRequest.md) |  | [required] |

### Return type

[**models::CreateAlertRule201Response**](createAlertRule_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

