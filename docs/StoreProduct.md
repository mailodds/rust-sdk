# StoreProduct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Product UUID | [optional]
**store_id** | Option<**String**> | Store connection UUID | [optional]
**external_id** | Option<**String**> | Product ID in the source store | [optional]
**sku** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**price_current** | Option<**f64**> | Current price | [optional]
**price_original** | Option<**f64**> | Original price (before discount) | [optional]
**currency** | Option<**String**> |  | [optional][default to USD]
**sale_start** | Option<**String**> |  | [optional]
**sale_end** | Option<**String**> |  | [optional]
**stock_status** | Option<**StockStatus**> |  (enum: in_stock, out_of_stock, on_backorder, preorder) | [optional]
**stock_quantity** | Option<**i32**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**additional_images** | Option<**Vec<String>**> |  | [optional]
**categories** | Option<**Vec<String>**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**product_url** | Option<**String**> |  | [optional]
**variants** | Option<**Vec<serde_json::Value>**> |  | [optional]
**is_active** | Option<**bool**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


