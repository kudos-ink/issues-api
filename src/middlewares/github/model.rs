

#[derive(Debug, Clone)]
pub struct GitHubUser {
    pub id: i64,
    pub username: String,
    pub avatar_url: String,
    pub email: Option<String>,
}