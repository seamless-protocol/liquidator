/*
 * ParaSwap API v5
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: contact@paraswap.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokensList {
    #[serde(rename = "tokens", skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<crate::models::Token>>,
}

impl TokensList {
    pub fn new() -> TokensList {
        TokensList {
            tokens: None,
        }
    }
}

