use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Repository {
    pub id: i32,
    pub name: String,
    pub organization_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RepositoryCreateRequest {
    pub name: String,
    pub organization_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RepositoryResponse {
    pub id: i32,
    pub name: String,
    pub organization_id: i32,
}

impl RepositoryResponse {
    pub fn of(repository: Repository) -> RepositoryResponse {
        RepositoryResponse {
            id: repository.id,
            name: repository.name,
            organization_id: repository.organization_id,
        }
    }
}
