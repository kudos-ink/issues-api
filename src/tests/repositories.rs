#[cfg(test)]
pub mod tests {
    use crate::repository::{
        db::DBRepository,
        models::{Repository, RepositoryRequest},
    };
    use mobc::async_trait;
    use warp::reject;

    #[derive(Clone)]
    pub struct RepositoriesDBMockValues {}
    #[derive(Clone)]
    pub struct RepositoriesDBMockEmpty {}

    #[async_trait]
    impl DBRepository for RepositoriesDBMockValues {
        async fn get_repository(&self, id: i32) -> Result<Option<Repository>, reject::Rejection> {
            Ok(Some(Repository {
                id,
                name: "repo".to_owned(),
                organization_id: 1,
            }))
        }
        async fn get_repository_by_name(
            &self,
            name: &str,
        ) -> Result<Option<Repository>, reject::Rejection> {
            Ok(Some(Repository {
                id: 1,
                name: name.to_owned(),
                organization_id: 1,
            }))
        }
        async fn get_repositories(&self) -> Result<Vec<Repository>, reject::Rejection> {
            Ok(vec![Repository {
                name: "repo".to_owned(),
                organization_id: 1,
                id: 1,
            }])
        }
        async fn create_repository(
            &self,
            repository: RepositoryRequest,
        ) -> Result<Repository, reject::Rejection> {
            Ok(Repository {
                name: repository.name.to_string(),
                id: 1,
                organization_id: repository.organization_id,
            })
        }
        async fn delete_repository(&self, _: i32) -> Result<(), reject::Rejection> {
            Ok(())
        }
    }
    #[async_trait]
    impl DBRepository for RepositoriesDBMockEmpty {
        async fn get_repository(&self, _: i32) -> Result<Option<Repository>, reject::Rejection> {
            Ok(None)
        }
        async fn get_repository_by_name(
            &self,
            _: &str,
        ) -> Result<Option<Repository>, reject::Rejection> {
            Ok(None)
        }
        async fn get_repositories(&self) -> Result<Vec<Repository>, reject::Rejection> {
            Ok(vec![])
        }
        async fn create_repository(
            &self,
            repository: RepositoryRequest,
        ) -> Result<Repository, reject::Rejection> {
            Ok(Repository {
                name: repository.name.to_string(),
                id: 1,
                organization_id: repository.organization_id,
            })
        }
        async fn delete_repository(&self, _: i32) -> Result<(), reject::Rejection> {
            Ok(())
        }
    }
}
