// use serde::Serialize;
// use serde_json::to_string as json;
// #[derive(Serialize)]
// pub struct Response {
//     code: u32,
//     body: String,
// }
// impl Response {
//     fn ok(msg: &str) -> Self {
//         Response {
//             code: 200,
//             body: msg.to_string(),
//         }
//     }
// }
#[get("/healthz")]
pub fn get() -> String {
    // json(&Response::ok("Genki").unwrap())
    "{}".into()
}
