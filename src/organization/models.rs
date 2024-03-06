use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Organization {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrganizationRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct OrganizationResponse {
    pub id: i32,
    pub name: String,
}

impl OrganizationResponse {
    pub fn of(organization: Organization) -> OrganizationResponse {
        OrganizationResponse {
            id: organization.id,
            name: organization.name,
        }
    }
}
