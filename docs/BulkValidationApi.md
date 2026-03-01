# \BulkValidationApi

All URIs are relative to *https://api.mailodds.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_job**](BulkValidationApi.md#cancel_job) | **POST** /v1/jobs/{job_id}/cancel | Cancel a job
[**create_job**](BulkValidationApi.md#create_job) | **POST** /v1/jobs | Create bulk validation job (JSON)
[**create_job_from_s3**](BulkValidationApi.md#create_job_from_s3) | **POST** /v1/jobs/upload/s3 | Create job from S3 upload
[**create_job_upload**](BulkValidationApi.md#create_job_upload) | **POST** /v1/jobs/upload | Create bulk validation job (file upload)
[**delete_job**](BulkValidationApi.md#delete_job) | **DELETE** /v1/jobs/{job_id} | Delete a job
[**get_job**](BulkValidationApi.md#get_job) | **GET** /v1/jobs/{job_id} | Get job status
[**get_job_results**](BulkValidationApi.md#get_job_results) | **GET** /v1/jobs/{job_id}/results | Get job results
[**get_presigned_upload**](BulkValidationApi.md#get_presigned_upload) | **POST** /v1/jobs/upload/presigned | Get S3 presigned upload URL
[**list_jobs**](BulkValidationApi.md#list_jobs) | **GET** /v1/jobs | List validation jobs



## cancel_job

> models::JobResponse cancel_job(job_id)
Cancel a job

Cancel a pending or processing job. Partial results are preserved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job

> models::JobResponse create_job(create_job_request)
Create bulk validation job (JSON)

Create a new bulk validation job by submitting a JSON array of emails.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_job_request** | [**CreateJobRequest**](CreateJobRequest.md) |  | [required] |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job_from_s3

> models::JobResponse create_job_from_s3(create_job_from_s3_request)
Create job from S3 upload

Create a validation job from a file previously uploaded to S3.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_job_from_s3_request** | [**CreateJobFromS3Request**](CreateJobFromS3Request.md) |  | [required] |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job_upload

> models::JobResponse create_job_upload(file, dedup, metadata)
Create bulk validation job (file upload)

Create a new bulk validation job by uploading a CSV, Excel, or TXT file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | CSV, Excel (.xlsx, .xls), ODS, or TXT file | [required] |
**dedup** | Option<**bool**> | Remove duplicate emails |  |[default to false]
**metadata** | Option<**String**> | JSON metadata for the job |  |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_job

> models::DeleteJob200Response delete_job(job_id)
Delete a job

Permanently delete a completed or cancelled job and its results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::DeleteJob200Response**](deleteJob_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job

> models::JobResponse get_job(job_id)
Get job status

Get the status and details of a specific validation job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_results

> models::ResultsResponse get_job_results(job_id, format, filter, page, per_page)
Get job results

Download validation results in JSON, CSV, or NDJSON format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**format** | Option<**String**> |  |  |[default to json]
**filter** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 1000]

### Return type

[**models::ResultsResponse**](ResultsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv, application/x-ndjson

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_presigned_upload

> models::PresignedUploadResponse get_presigned_upload(get_presigned_upload_request)
Get S3 presigned upload URL

Get a presigned URL for uploading large files (>10MB) directly to S3.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_presigned_upload_request** | [**GetPresignedUploadRequest**](GetPresignedUploadRequest.md) |  | [required] |

### Return type

[**models::PresignedUploadResponse**](PresignedUploadResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_jobs

> models::JobListResponse list_jobs(cursor, limit, status)
List validation jobs

List all validation jobs for the authenticated account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Pagination cursor (ISO timestamp from previous response) |  |
**limit** | Option<**i32**> | Results per page |  |[default to 50]
**status** | Option<**String**> |  |  |

### Return type

[**models::JobListResponse**](JobListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

