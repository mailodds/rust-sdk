# \ValidationPoliciesApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_policy_rule**](ValidationPoliciesApi.md#add_policy_rule) | **POST** /v1/policies/{policy_id}/rules | Add rule to policy
[**create_policy**](ValidationPoliciesApi.md#create_policy) | **POST** /v1/policies | Create policy
[**create_policy_from_preset**](ValidationPoliciesApi.md#create_policy_from_preset) | **POST** /v1/policies/from-preset | Create policy from preset
[**delete_policy**](ValidationPoliciesApi.md#delete_policy) | **DELETE** /v1/policies/{policy_id} | Delete policy
[**delete_policy_rule**](ValidationPoliciesApi.md#delete_policy_rule) | **DELETE** /v1/policies/{policy_id}/rules/{rule_id} | Delete rule
[**get_policy**](ValidationPoliciesApi.md#get_policy) | **GET** /v1/policies/{policy_id} | Get policy
[**get_policy_presets**](ValidationPoliciesApi.md#get_policy_presets) | **GET** /v1/policies/presets | Get policy presets
[**list_policies**](ValidationPoliciesApi.md#list_policies) | **GET** /v1/policies | List policies
[**test_policy**](ValidationPoliciesApi.md#test_policy) | **POST** /v1/policies/test | Test policy evaluation
[**update_policy**](ValidationPoliciesApi.md#update_policy) | **PUT** /v1/policies/{policy_id} | Update policy



## add_policy_rule

> models::AddPolicyRule201Response add_policy_rule(policy_id, policy_rule)
Add rule to policy

Add a new rule to an existing policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** |  | [required] |
**policy_rule** | [**PolicyRule**](PolicyRule.md) |  | [required] |

### Return type

[**models::AddPolicyRule201Response**](addPolicyRule_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_policy

> models::PolicyResponse create_policy(create_policy_request)
Create policy

Create a new validation policy with rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_policy_request** | [**CreatePolicyRequest**](CreatePolicyRequest.md) |  | [required] |

### Return type

[**models::PolicyResponse**](PolicyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_policy_from_preset

> models::PolicyResponse create_policy_from_preset(create_policy_from_preset_request)
Create policy from preset

Create a policy using a preset template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_policy_from_preset_request** | [**CreatePolicyFromPresetRequest**](CreatePolicyFromPresetRequest.md) |  | [required] |

### Return type

[**models::PolicyResponse**](PolicyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy

> models::DeletePolicy200Response delete_policy(policy_id)
Delete policy

Delete a policy and all its rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** |  | [required] |

### Return type

[**models::DeletePolicy200Response**](deletePolicy_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy_rule

> models::DeletePolicyRule200Response delete_policy_rule(policy_id, rule_id)
Delete rule

Delete a rule from a policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** |  | [required] |
**rule_id** | **i32** |  | [required] |

### Return type

[**models::DeletePolicyRule200Response**](deletePolicyRule_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy

> models::PolicyResponse get_policy(policy_id)
Get policy

Get a single policy with its rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** |  | [required] |

### Return type

[**models::PolicyResponse**](PolicyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy_presets

> models::PolicyPresetsResponse get_policy_presets()
Get policy presets

Get available preset templates for quick policy creation.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PolicyPresetsResponse**](PolicyPresetsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_policies

> models::PolicyListResponse list_policies(include_rules)
List policies

List all validation policies for your account. Includes plan limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_rules** | Option<**bool**> | Include full rules in response |  |[default to false]

### Return type

[**models::PolicyListResponse**](PolicyListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_policy

> models::PolicyTestResponse test_policy(test_policy_request)
Test policy evaluation

Test how a policy would evaluate a validation result without affecting production.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_policy_request** | [**TestPolicyRequest**](TestPolicyRequest.md) |  | [required] |

### Return type

[**models::PolicyTestResponse**](PolicyTestResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_policy

> models::PolicyResponse update_policy(policy_id, update_policy_request)
Update policy

Update a policy's settings (name, enabled, default).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** |  | [required] |
**update_policy_request** | [**UpdatePolicyRequest**](UpdatePolicyRequest.md) |  | [required] |

### Return type

[**models::PolicyResponse**](PolicyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

