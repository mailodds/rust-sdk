# SendingDomain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Domain UUID | [optional]
**domain** | Option<**String**> | Domain name | [optional]
**domain_type** | Option<**String**> | Domain type (ns_delegated) | [optional]
**status** | Option<**Status**> | Domain verification status (enum: pending_dns, dns_partial, active, suspended) | [optional]
**dkim_selector** | Option<**String**> | DKIM selector (e.g. mo1) | [optional]
**dns_records** | Option<[**models::SendingDomainDnsRecords**](SendingDomainDnsRecords.md)> |  | [optional]
**bimi_svg_url** | Option<**String**> | BIMI SVG logo URL | [optional]
**bimi_vmc_url** | Option<**String**> | BIMI VMC certificate URL | [optional]
**bimi_enabled** | Option<**bool**> | Whether BIMI is enabled | [optional]
**forward_replies_to** | Option<**String**> | Reply forwarding address | [optional]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


