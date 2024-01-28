use serde::Serialize;
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
    serde_json::to_string(&response).unwrap()
}
