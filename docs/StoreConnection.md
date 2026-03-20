# StoreConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Store connection UUID | [optional]
**account_id** | Option<**i32**> |  | [optional]
**platform** | Option<**Platform**> | E-commerce platform (enum: woocommerce, prestashop, shopify, feed, custom) | [optional]
**store_name** | Option<**String**> |  | [optional]
**store_url** | Option<**String**> |  | [optional]
**status** | Option<**Status**> |  (enum: pending, connected, active, syncing, error, disconnected) | [optional]
**auth_method** | Option<**AuthMethod**> |  (enum: plugin_handshake, api_key, oauth, feed_url) | [optional]
**product_count** | Option<**i32**> | Number of active products | [optional]
**last_synced_at** | Option<**String**> |  | [optional]
**last_error** | Option<**String**> | Last sync error message | [optional]
**sync_interval_seconds** | Option<**i32**> | Auto-sync interval in seconds | [optional][default to 3600]
**settings** | Option<**serde_json::Value**> | Platform-specific settings | [optional]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


