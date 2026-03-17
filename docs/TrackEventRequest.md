# TrackEventRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_type** | **EventType** | Type of commerce event (enum: purchase, cart_abandon, browse, wishlist, review, shadow_dns_dwell, css_probe, identity_stitch) | 
**email** | **String** | Email address associated with the event | 
**properties** | Option<**serde_json::Value**> | Event-specific data (e.g., order_id, value, product_url) | [optional]
**occurred_at** | Option<**String**> | When the event occurred (defaults to now) | [optional]
**idempotency_key** | Option<**String**> | Unique key to prevent duplicate events (5 min TTL) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


