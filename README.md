# Rust API client for memas-sdk

This is the Control Plane client for MeMaS (Memory Management Service). 
See https://github.com/memas-ai/MeMaS for more details.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.1.0
- Package version: 0.1.2
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `memas-sdk` and add the following to `Cargo.toml` under `[dependencies]`:

```
memas-sdk = { path = "./memas-sdk" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CpApi* | [**create_corpus**](docs/CpApi.md#create_corpus) | **POST** /cp/create_corpus | Create corpus
*CpApi* | [**create_user**](docs/CpApi.md#create_user) | **POST** /cp/create_user | Create user


## Documentation For Models

 - [CorpusType](docs/CorpusType.md)
 - [CreateCorpusRequest](docs/CreateCorpusRequest.md)
 - [CreateUser200Response](docs/CreateUser200Response.md)
 - [CreateUser400Response](docs/CreateUser400Response.md)
 - [CreateUserRequest](docs/CreateUserRequest.md)
 - [NamespaceDoesNotExistError](docs/NamespaceDoesNotExistError.md)
 - [NamespaceExistError](docs/NamespaceExistError.md)
 - [NamespaceIllegalNameError](docs/NamespaceIllegalNameError.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

max.yu@memas.ai

