use std::sync::Arc;

use crate::{db::DbPool, models::accounts};

#[derive(Clone)]
pub struct Context {
    pub db_pool: Arc<DbPool>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Context { db_pool }
    }
}
