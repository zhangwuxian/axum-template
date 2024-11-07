use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub code: u64,
    pub data: T,
}

pub fn success_response<T: Serialize>(data: T) -> String {
    let resp = Response { code: 200, data };
    serde_json::to_string(&resp).unwrap()
}

pub fn error_response(err: String) -> String {
    let resp = Response {
        code: 500,
        data: err,
    };
    serde_json::to_string(&resp).unwrap()
}
