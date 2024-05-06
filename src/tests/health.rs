#[cfg(test)]
mod tests {
    use crate::{
        api::health::{db::DBHealth, routes::routes},
        db::errors::DBError,
        types::ApiConfig,
        utils::setup_db,
    };
    use warp::test::request;

    #[derive(Clone)]
    pub struct DBMock {}

    impl DBHealth for DBMock {
        fn health(&self) -> Result<(), DBError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_health_mock_db() {
        let r = routes(DBMock {});
        let resp = request().path("/health").reply(&r).await;
        assert_eq!(resp.status(), 200);
        assert!(resp.body().is_empty());
    }

    #[tokio::test]
    async fn test_health_db() {
        let ApiConfig { database_url, .. } = ApiConfig::new();
        let db = setup_db(&database_url).await;
        let r = routes(db);
        let resp = request().path("/health").reply(&r).await;
        assert_eq!(resp.status(), 200);
        assert!(resp.body().is_empty());
    }
}
