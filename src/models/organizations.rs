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
pub struct Organizations {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::Organization>,
    /// Total number of organizations 
    #[serde(rename = "total_count")]
    pub total_count: i64,
}

impl Organizations {
    pub fn new(data: Vec<crate::models::Organization>, total_count: i64) -> Organizations {
        Organizations {
            data,
            total_count,
        }
    }
}


