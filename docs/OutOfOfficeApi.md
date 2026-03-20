# \OutOfOfficeApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_check_ooo**](OutOfOfficeApi.md#batch_check_ooo) | **POST** /v1/out-of-office/batch-check | Batch check OOO status
[**delete_ooo_contact**](OutOfOfficeApi.md#delete_ooo_contact) | **DELETE** /v1/out-of-office/{email} | Delete OOO contact
[**get_ooo_status**](OutOfOfficeApi.md#get_ooo_status) | **GET** /v1/out-of-office/{email}/status | Get OOO status for email
[**list_ooo_contacts**](OutOfOfficeApi.md#list_ooo_contacts) | **GET** /v1/out-of-office | List out-of-office contacts
[**update_ooo_contact**](OutOfOfficeApi.md#update_ooo_contact) | **PATCH** /v1/out-of-office/{email} | Update OOO contact



## batch_check_ooo

> models::BatchCheckOoo200Response batch_check_ooo(batch_check_ooo_request)
Batch check OOO status

Check OOO status for up to 1000 email addresses at once. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_check_ooo_request** | [**BatchCheckOooRequest**](BatchCheckOooRequest.md) |  | [required] |

### Return type

[**models::BatchCheckOoo200Response**](batchCheckOoo_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ooo_contact

> models::DeleteOooContact200Response delete_ooo_contact(email)
Delete OOO contact

Clear out-of-office status for an email address. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |

### Return type

[**models::DeleteOooContact200Response**](deleteOooContact_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ooo_status

> models::GetOooStatus200Response get_ooo_status(email)
Get OOO status for email

Check if a specific email address is currently out-of-office. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |

### Return type

[**models::GetOooStatus200Response**](getOooStatus_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ooo_contacts

> models::ListOooContacts200Response list_ooo_contacts(active_only, page, per_page)
List out-of-office contacts

List contacts detected as out-of-office. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**active_only** | Option<**bool**> | Only return currently active OOO contacts |  |[default to true]
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 100]

### Return type

[**models::ListOooContacts200Response**](listOooContacts_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ooo_contact

> serde_json::Value update_ooo_contact(email, update_ooo_contact_request)
Update OOO contact

Manually set or clear out-of-office status for an email. Requires Growth+ plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**update_ooo_contact_request** | [**UpdateOooContactRequest**](UpdateOooContactRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

