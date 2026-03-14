# \ContactListsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**append_to_contact_list**](ContactListsApi.md#append_to_contact_list) | **POST** /v1/contact-lists/{list_id}/append | Append to contact list
[**create_contact_list**](ContactListsApi.md#create_contact_list) | **POST** /v1/contact-lists | Create contact list
[**get_inactive_contacts_report**](ContactListsApi.md#get_inactive_contacts_report) | **GET** /v1/contacts/inactive-report | Get inactive contacts report
[**list_contact_lists**](ContactListsApi.md#list_contact_lists) | **GET** /v1/contact-lists | List contact lists
[**query_contact_list**](ContactListsApi.md#query_contact_list) | **POST** /v1/contact-lists/{list_id}/query | Query contact list



## append_to_contact_list

> models::AppendToContactList200Response append_to_contact_list(list_id, append_to_contact_list_request)
Append to contact list

Append validated emails from additional jobs to an existing contact list. Duplicates are automatically skipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** | Contact list UUID | [required] |
**append_to_contact_list_request** | [**AppendToContactListRequest**](AppendToContactListRequest.md) |  | [required] |

### Return type

[**models::AppendToContactList200Response**](appendToContactList_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_contact_list

> models::CreateContactList201Response create_contact_list(create_contact_list_request)
Create contact list

Create a new contact list from one or more completed validation jobs. Only accepted (valid) emails are included.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_contact_list_request** | [**CreateContactListRequest**](CreateContactListRequest.md) |  | [required] |

### Return type

[**models::CreateContactList201Response**](createContactList_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inactive_contacts_report

> models::GetInactiveContactsReport200Response get_inactive_contacts_report(days)
Get inactive contacts report

Get a report of contacts across all lists with no engagement activity (opens, clicks) in the specified period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> | Inactivity threshold in days |  |[default to 90]

### Return type

[**models::GetInactiveContactsReport200Response**](getInactiveContactsReport_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_contact_lists

> models::ListContactLists200Response list_contact_lists(page, per_page)
List contact lists

List contact lists for the authenticated account. Contact lists are built from validated email jobs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ListContactLists200Response**](listContactLists_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_contact_list

> models::QueryContactList200Response query_contact_list(list_id, query_contact_list_request)
Query contact list

Query contact list entries with structured filters. Supports filtering by validation status, domain, and other attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** | Contact list UUID | [required] |
**query_contact_list_request** | [**QueryContactListRequest**](QueryContactListRequest.md) |  | [required] |

### Return type

[**models::QueryContactList200Response**](queryContactList_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

