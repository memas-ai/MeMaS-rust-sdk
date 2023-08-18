/*
 * MeMaS CP APIs
 *
 * This is the Control Plane APIs for MeMaS (Memory Management Service).
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: max.yu@memas.ai
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// \"Full namespace name, where child namespaces are appended after their parents' names with '.'\"
    #[serde(rename = "namespace_pathname", skip_serializing_if = "Option::is_none")]
    pub namespace_pathname: Option<String>,
}

impl CreateUserRequest {
    pub fn new() -> CreateUserRequest {
        CreateUserRequest {
            namespace_pathname: None,
        }
    }
}


