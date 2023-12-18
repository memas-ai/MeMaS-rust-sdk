# \CpApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_corpus**](CpApi.md#create_corpus) | **POST** /cp/corpus | Create corpus
[**create_user**](CpApi.md#create_user) | **POST** /cp/user | Create user
[**delete_corpus**](CpApi.md#delete_corpus) | **DELETE** /cp/corpus | Delete corpus
[**delete_user**](CpApi.md#delete_user) | **DELETE** /cp/user | Delete user



## create_corpus

> crate::models::Memorize200Response create_corpus(create_corpus_request)
Create corpus

Create a corpus under a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_corpus_request** | [**CreateCorpusRequest**](CreateCorpusRequest.md) |  | [required] |

### Return type

[**crate::models::Memorize200Response**](memorize_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::Memorize200Response create_user(create_user_request)
Create user

Create a namespace user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**crate::models::Memorize200Response**](memorize_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_corpus

> crate::models::Memorize200Response delete_corpus(delete_corpus_request)
Delete corpus

Delete a corpus under a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_corpus_request** | [**DeleteCorpusRequest**](DeleteCorpusRequest.md) |  | [required] |

### Return type

[**crate::models::Memorize200Response**](memorize_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> crate::models::Memorize200Response delete_user(create_user_request)
Delete user

Delete a namespace user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**crate::models::Memorize200Response**](memorize_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

