# \OAuth20Api

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_token**](OAuth20Api.md#create_token) | **POST** /oauth/token | Create token
[**get_jwks**](OAuth20Api.md#get_jwks) | **GET** /.well-known/jwks.json | Get JSON Web Key Set
[**introspect_token**](OAuth20Api.md#introspect_token) | **POST** /oauth/introspect | Introspect token
[**oauth_server_metadata**](OAuth20Api.md#oauth_server_metadata) | **GET** /.well-known/oauth-authorization-server | OAuth server metadata
[**revoke_token**](OAuth20Api.md#revoke_token) | **POST** /oauth/revoke | Revoke token



## create_token

> models::CreateToken200Response create_token(grant_type, code, redirect_uri, client_id, client_secret, refresh_token, scope, code_verifier)
Create token

Exchange an authorization code, client credentials, or refresh token for access and refresh tokens. Authenticate via client_secret_post or client_secret_basic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** |  | [required] |
**code** | Option<**String**> | Authorization code (for authorization_code grant) |  |
**redirect_uri** | Option<**String**> | Must match the original redirect_uri |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |
**refresh_token** | Option<**String**> | Refresh token (for refresh_token grant) |  |
**scope** | Option<**String**> | Space-separated scopes |  |
**code_verifier** | Option<**String**> | PKCE code verifier |  |

### Return type

[**models::CreateToken200Response**](createToken_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jwks

> models::JwksResponse get_jwks()
Get JSON Web Key Set

Public key set for verifying JWT access tokens issued by this server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::JwksResponse**](JwksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## introspect_token

> models::IntrospectToken200Response introspect_token(token, token_type_hint, client_id, client_secret)
Introspect token

Introspect a token to determine its active state and metadata (RFC 7662). Requires client authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Token to introspect | [required] |
**token_type_hint** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |

### Return type

[**models::IntrospectToken200Response**](introspectToken_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_server_metadata

> models::OAuthServerMetadata oauth_server_metadata()
OAuth server metadata

OAuth 2.0 Authorization Server Metadata (RFC 8414). Returns server configuration including supported grant types, scopes, and endpoints.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OAuthServerMetadata**](OAuthServerMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_token

> revoke_token(token, token_type_hint, client_id, client_secret)
Revoke token

Revoke an access or refresh token (RFC 7009). Requires client authentication. Always returns 200 per spec to prevent token scanning.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Token to revoke | [required] |
**token_type_hint** | Option<**String**> |  |  |
**client_id** | Option<**String**> |  |  |
**client_secret** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

