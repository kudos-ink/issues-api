#[cfg(test)]
mod tests {
    use crate::{
        health::{db::DBHealth, routes::routes},
        init_db,
    };
    use mobc::async_trait;
    use warp::{reject, test::request};

    #[derive(Clone)]
    pub struct DBMock {}

    #[async_trait]
    impl DBHealth for DBMock {
        async fn health(&self) -> Result<(), reject::Rejection> {
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
        let db = init_db(
            "postgres://postgres:password@localhost:5432/database".to_string(),
            "db.sql".to_string(),
        )
        .await;

        let r = routes(db);
        let resp = request().path("/health").reply(&r).await;
        assert_eq!(resp.status(), 200);
        assert!(resp.body().is_empty());
    }
}

// TODO: add e2e test using a real http server.
// hyper can be used for that
