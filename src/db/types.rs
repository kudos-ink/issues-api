use diesel::r2d2::PooledConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DBConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
