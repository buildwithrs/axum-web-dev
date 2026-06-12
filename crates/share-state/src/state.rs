use std::{collections::HashMap, ops::Deref, sync::Arc};

use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

impl AppState {
    pub fn new(redis_conn: redis::Client) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                redis_conn: redis_conn,
                cache: RwLock::new(HashMap::with_capacity(1000)),
            }),
        }
    }
}

pub struct AppStateInner {
    pub redis_conn: redis::Client,
    pub cache: RwLock<HashMap<String, Vec<u8>>>,
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
