/*
 * MeMaS APIs
 *
 * This is the Control Plane and Data Plane APIs for MeMaS (Memory Management Service). See https://github.com/memas-ai/MeMaS for more details.
 *
 * The version of the OpenAPI document: 0.1.1
 * Contact: max.yu@memas.ai
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateUser400Response {
    #[serde(rename = "error_code")]
    pub error_code: ErrorCode,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl CreateUser400Response {
    pub fn new(error_code: ErrorCode, msg: String) -> CreateUser400Response {
        CreateUser400Response {
            error_code,
            msg,
            details: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "namespace_illegal_name")]
    NamespaceIllegalName,
}

impl Default for ErrorCode {
    fn default() -> ErrorCode {
        Self::NamespaceIllegalName
    }
}

