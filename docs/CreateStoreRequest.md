# CreateStoreRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**platform** | **Platform** | E-commerce platform (enum: woocommerce, prestashop, shopify, feed, custom) | 
**store_name** | **String** | Display name for the store | 
**store_url** | **String** | Store base URL | 
**auth_method** | **AuthMethod** | Authentication method (enum: plugin_handshake, api_key, oauth, feed_url) | 
**settings** | Option<**serde_json::Value**> | Platform-specific settings (e.g., API keys, feed URL) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


