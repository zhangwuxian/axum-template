use dashmap::DashMap;

#[derive(Clone, Default)]
pub struct HttpServerState {
    pub cache: DashMap<String, String>,
}
