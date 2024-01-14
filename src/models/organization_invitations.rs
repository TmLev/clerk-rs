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
pub struct OrganizationInvitations {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::OrganizationInvitation>,
    /// Total number of organization invitations 
    #[serde(rename = "total_count")]
    pub total_count: i64,
}

impl OrganizationInvitations {
    pub fn new(data: Vec<crate::models::OrganizationInvitation>, total_count: i64) -> OrganizationInvitations {
        OrganizationInvitations {
            data,
            total_count,
        }
    }
}


