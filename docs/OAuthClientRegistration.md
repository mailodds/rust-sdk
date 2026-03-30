# OAuthClientRegistration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **String** | Issued client identifier | 
**client_name** | **String** | Human-readable client name | 
**redirect_uris** | **Vec<String>** | Registered redirect URIs | 
**grant_types** | **Vec<String>** | Allowed grant types | 
**response_types** | **Vec<String>** | Allowed response types | 
**token_endpoint_auth_method** | **String** | Token endpoint auth method | 
**scope** | Option<**String**> | Allowed scope | [optional]
**client_id_issued_at** | **i32** | Unix timestamp of client registration | 
**client_secret** | Option<**String**> | Client secret (only for confidential clients, shown once) | [optional]
**client_secret_expires_at** | Option<**i32**> | Secret expiry (0 = never) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


