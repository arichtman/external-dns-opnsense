use serde::Serialize;
use serde_json::to_string as json;
#[derive(Serialize)]
struct ResponseBody {
    key: String,
    another_key: u8,
}

#[get("/")]
pub fn get() -> String {
    let response = ResponseBody {
        key: "value".into(),
        another_key: 5,
    };
    json(&response).unwrap()
}
