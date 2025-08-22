use sqlx::postgres;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: sqlx::Pool<postgres::Postgres>,
}
