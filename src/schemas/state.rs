use std::sync::Arc;

use crate::api::{create_schema, Schema};
use crate::errors::AppError;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppData {
    pub db: PgPool,
    pub schema: Arc<Schema>,
}

impl AppData {
    pub fn init() -> Result<Self, AppError> {
        let database_url = dotenv::var("DATABASE_URL").expect("Set DATABASE_URL in .env file");
        let pool = PgPool::connect_lazy(&database_url)?;
        log::info!("Database connection establish successful");

        let schema = Arc::new(create_schema());
        Ok(AppData { db: pool, schema })
    }
}
