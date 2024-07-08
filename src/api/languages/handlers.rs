use super::{db::DBLanguage, models::NewLanguage};
use warp::{
    reject::Rejection,
    reply::{json, Reply},
};

pub async fn all_handler(db_access: impl DBLanguage) -> Result<impl Reply, Rejection> {
    let languages = db_access.all()?;
    Ok(json::<Vec<_>>(&languages))
}

pub async fn create_handler(
    form: NewLanguage,
    db_access: impl DBLanguage,
) -> Result<impl Reply, warp::Rejection> {
    let language = db_access.create_or_get(&form)?;
    Ok(json(&language))
}
