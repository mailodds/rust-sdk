# \ContactListsApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_contact**](ContactListsApi.md#add_contact) | **POST** /v1/contact-lists/{list_id}/contacts | Add contact to list
[**append_to_contact_list**](ContactListsApi.md#append_to_contact_list) | **POST** /v1/contact-lists/{list_id}/append | Append to contact list
[**create_contact_list**](ContactListsApi.md#create_contact_list) | **POST** /v1/contact-lists | Create contact list
[**delete_contact**](ContactListsApi.md#delete_contact) | **DELETE** /v1/contact-lists/{list_id}/contacts/{contact_id} | Delete contact
[**delete_contact_list**](ContactListsApi.md#delete_contact_list) | **DELETE** /v1/contact-lists/{list_id} | Delete a contact list
[**export_contact_list**](ContactListsApi.md#export_contact_list) | **GET** /v1/contact-lists/{list_id}/export | Export contact list
[**get_inactive_contacts_report**](ContactListsApi.md#get_inactive_contacts_report) | **GET** /v1/contacts/inactive-report | Get inactive contacts report
[**import_contact_list**](ContactListsApi.md#import_contact_list) | **POST** /v1/contact-lists/{list_id}/import | Import contacts from CSV
[**list_contact_lists**](ContactListsApi.md#list_contact_lists) | **GET** /v1/contact-lists | List contact lists
[**query_contact_list**](ContactListsApi.md#query_contact_list) | **POST** /v1/contact-lists/{list_id}/query | Query contact list
[**update_contact**](ContactListsApi.md#update_contact) | **PATCH** /v1/contact-lists/{list_id}/contacts/{contact_id} | Update contact



## add_contact

> models::AddContact201Response add_contact(list_id, add_contact_request)
Add contact to list

Add a single contact to a contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | [required] |
**add_contact_request** | [**AddContactRequest**](AddContactRequest.md) |  | [required] |

### Return type

[**models::AddContact201Response**](addContact_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## append_to_contact_list

> models::AppendToContactList200Response append_to_contact_list(list_id, append_to_contact_list_request)
Append to contact list

Append validated emails from additional jobs to an existing contact list. Duplicates are automatically skipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | [required] |
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


## delete_contact

> models::DeletePolicyRule200Response delete_contact(list_id, contact_id)
Delete contact

Remove a single contact from a contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | [required] |
**contact_id** | **String** |  | [required] |

### Return type

[**models::DeletePolicyRule200Response**](deletePolicyRule_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contact_list

> models::DeletePolicyRule200Response delete_contact_list(list_id)
Delete a contact list

Permanently delete a contact list and all its entries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | [required] |

### Return type

[**models::DeletePolicyRule200Response**](deletePolicyRule_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_contact_list

> String export_contact_list(list_id)
Export contact list

Export a contact list as CSV.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/json

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


## import_contact_list

> models::ImportContactList200Response import_contact_list(list_id, file, column_mapping, consent_source, tags)
Import contacts from CSV

Import contacts into a list from a CSV file (max 10MB).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | [required] |
**file** | **std::path::PathBuf** | CSV file (max 10MB) | [required] |
**column_mapping** | Option<**String**> | JSON mapping of CSV columns to contact fields |  |
**consent_source** | Option<**String**> | Source of consent for imported contacts |  |
**tags** | Option<**String**> | JSON array of tags to apply |  |

### Return type

[**models::ImportContactList200Response**](importContactList_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
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
**list_id** | **String** |  | [required] |
**query_contact_list_request** | [**QueryContactListRequest**](QueryContactListRequest.md) |  | [required] |

### Return type

[**models::QueryContactList200Response**](queryContactList_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_contact

> models::AddContact201Response update_contact(list_id, contact_id, update_contact_request)
Update contact

Update a single contact in a contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **String** |  | [required] |
**contact_id** | **String** |  | [required] |
**update_contact_request** | [**UpdateContactRequest**](UpdateContactRequest.md) |  | [required] |

### Return type

[**models::AddContact201Response**](addContact_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

