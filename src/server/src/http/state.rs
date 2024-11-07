use dashmap::DashMap;

#[derive(Clone)]
pub struct HttpServerState {
    pub cache: DashMap<String, String>,
    pub jwt_secret: String,
}

impl HttpServerState {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            cache: DashMap::with_capacity(8),
            jwt_secret,
        }
    }
}
