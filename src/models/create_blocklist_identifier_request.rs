/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateBlocklistIdentifierRequest {
    /// The identifier to be added in the block-list. This can be an email address, a phone number or a web3 wallet.
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl CreateBlocklistIdentifierRequest {
    pub fn new(identifier: String) -> CreateBlocklistIdentifierRequest {
        CreateBlocklistIdentifierRequest {
            identifier,
        }
    }
}


