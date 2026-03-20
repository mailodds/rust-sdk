# \ReputationPoliciesApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_reputation_policy**](ReputationPoliciesApi.md#create_reputation_policy) | **POST** /v1/reputation-policies | Create a reputation policy
[**create_reputation_policy_from_preset**](ReputationPoliciesApi.md#create_reputation_policy_from_preset) | **POST** /v1/reputation-policies/from-preset | Create a reputation policy from preset
[**delete_reputation_policy**](ReputationPoliciesApi.md#delete_reputation_policy) | **DELETE** /v1/reputation-policies/{policy_id} | Delete a reputation policy
[**get_reputation_policy**](ReputationPoliciesApi.md#get_reputation_policy) | **GET** /v1/reputation-policies/{policy_id} | Get a reputation policy
[**get_reputation_policy_status**](ReputationPoliciesApi.md#get_reputation_policy_status) | **GET** /v1/reputation-policies/{policy_id}/status | Get reputation policy status
[**list_reputation_policies**](ReputationPoliciesApi.md#list_reputation_policies) | **GET** /v1/reputation-policies | List reputation policies
[**test_reputation_policy**](ReputationPoliciesApi.md#test_reputation_policy) | **POST** /v1/reputation-policies/{policy_id}/test | Test a reputation policy
[**update_reputation_policy**](ReputationPoliciesApi.md#update_reputation_policy) | **PUT** /v1/reputation-policies/{policy_id} | Update a reputation policy



## create_reputation_policy

> create_reputation_policy()
Create a reputation policy

Create a new reputation policy with custom rules and thresholds.

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


## create_reputation_policy_from_preset

> create_reputation_policy_from_preset()
Create a reputation policy from preset

Create a reputation policy from a predefined preset template.

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


## delete_reputation_policy

> delete_reputation_policy(policy_id)
Delete a reputation policy

Soft-delete a reputation policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reputation_policy

> get_reputation_policy(policy_id)
Get a reputation policy

Retrieve a single reputation policy by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reputation_policy_status

> get_reputation_policy_status(policy_id)
Get reputation policy status

Evaluate a policy and return per-domain status results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_reputation_policies

> list_reputation_policies()
List reputation policies

List all reputation policies for the account.

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


## test_reputation_policy

> test_reputation_policy(policy_id)
Test a reputation policy

Dry-run evaluation of a reputation policy without applying actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_reputation_policy

> update_reputation_policy(policy_id)
Update a reputation policy

Update an existing reputation policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

