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
pub struct RecallRequest {
    #[serde(rename = "clue")]
    pub clue: String,
    /// Full name of a corpus, specifying which namespace the corpus is under.  The name takes on the format of \\\"<namespace_pathname>:<corpus_name>\\\"
    #[serde(rename = "corpus_pathname")]
    pub corpus_pathname: String,
}

impl RecallRequest {
    pub fn new(clue: String, corpus_pathname: String) -> RecallRequest {
        RecallRequest {
            clue,
            corpus_pathname,
        }
    }
}

