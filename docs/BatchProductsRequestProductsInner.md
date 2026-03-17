# BatchProductsRequestProductsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external_id** | **String** |  | 
**title** | **String** |  | 
**product_url** | **String** |  | 
**sku** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**price_current** | Option<**f64**> |  | [optional]
**price_original** | Option<**f64**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**stock_status** | Option<**StockStatus**> |  (enum: in_stock, out_of_stock, on_backorder, preorder) | [optional]
**stock_quantity** | Option<**i32**> |  | [optional]
**image_url** | Option<**String**> |  | [optional]
**additional_images** | Option<**Vec<String>**> |  | [optional]
**categories** | Option<**Vec<String>**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**variants** | Option<**Vec<serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


