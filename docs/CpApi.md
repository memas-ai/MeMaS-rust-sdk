# \CpApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_corpus**](CpApi.md#create_corpus) | **POST** /cp/create_corpus | Create corpus
[**create_user**](CpApi.md#create_user) | **POST** /cp/create_user | Create user



## create_corpus

> crate::models::CreateUser200Response create_corpus(create_corpus_request)
Create corpus

Create a corpus under a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_corpus_request** | [**CreateCorpusRequest**](CreateCorpusRequest.md) |  | [required] |

### Return type

[**crate::models::CreateUser200Response**](createUser_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::CreateUser200Response create_user(create_user_request)
Create user

Create a namespace user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**crate::models::CreateUser200Response**](createUser_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

