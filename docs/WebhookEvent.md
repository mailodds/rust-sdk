# WebhookEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event** | **Event** | Event type (enum: job.completed, job.failed, message.queued, message.delivered, message.bounced, message.deferred, message.failed, message.opened, message.clicked, test) | 
**timestamp** | **String** | When the event occurred | 
**job** | Option<[**models::Job**](Job.md)> |  | [optional]
**message_id** | Option<**String**> | Message ID (present for message.* and delivery events) | [optional]
**account_id** | Option<**i32**> | Account ID (present for delivery events) | [optional]
**domain_id** | Option<**String**> | Sending domain UUID (present for delivery events) | [optional]
**to** | Option<**String**> | Recipient email (present for delivery events) | [optional]
**status** | Option<**String**> | Delivery status (present for delivery events) | [optional]
**smtp_code** | Option<**i32**> | SMTP response code (present for bounced/deferred/failed) | [optional]
**smtp_response** | Option<**String**> | SMTP diagnostic string (present for bounced/deferred/failed) | [optional]
**mx_host** | Option<**String**> | MX host that handled delivery | [optional]
**bounce_type** | Option<**BounceType**> | Bounce classification (present for message.bounced) (enum: hard, soft) | [optional]
**enhanced_status_code** | Option<**String**> | Enhanced SMTP status code (e.g., 5.1.1) | [optional]
**attempts** | Option<**i32**> | Number of delivery attempts | [optional]
**isp** | Option<**String**> | Receiving ISP name | [optional]
**is_mpp** | Option<**bool**> | Whether the open was from Apple Mail Privacy Protection | [optional]
**ip_address** | Option<**String**> | Client IP (present for message.opened/clicked) | [optional]
**user_agent** | Option<**String**> | Client user agent (present for message.opened/clicked) | [optional]
**is_bot** | Option<**bool**> | Whether the event was triggered by a bot (present for message.opened/clicked) | [optional]
**link_url** | Option<**String**> | Clicked URL (present for message.clicked) | [optional]
**link_index** | Option<**i32**> | Position of clicked link in message (present for message.clicked) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


