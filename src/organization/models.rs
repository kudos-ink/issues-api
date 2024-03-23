use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Organization {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrganizationQuery {
	pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OrganizationCreateRequest {
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
