#[cfg(test)]
mod tests {
    use crate::{
        api::health::{db::DBHealth, routes::routes},
        db::errors::DBError,
        utils::generate_test_database,
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
    #[ignore]
    async fn test_health_db() {
        let db = generate_test_database().await;
        let r = routes(db);
        let resp = request().path("/health").reply(&r).await;
        assert_eq!(resp.status(), 200);
        assert!(resp.body().is_empty());
    }
}
