# \DpApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**memorize**](DpApi.md#memorize) | **POST** /dp/memorize | Memorize information
[**recall**](DpApi.md#recall) | **GET** /dp/recall | Recalls information



## memorize

> crate::models::Memorize200Response memorize(memorize_request)
Memorize information

Memorize information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**memorize_request** | [**MemorizeRequest**](MemorizeRequest.md) |  | [required] |

### Return type

[**crate::models::Memorize200Response**](memorize_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recall

> Vec<crate::models::CitedDocument> recall(recall_request)
Recalls information

Recalls relevant information related to the given clue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recall_request** | [**RecallRequest**](RecallRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::CitedDocument>**](CitedDocument.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

