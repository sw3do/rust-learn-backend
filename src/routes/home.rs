use axum::Json;
use serde_json::{Value, json};

pub async fn homepage() -> Json<Value> {
    Json(json!({
        "ok": true,
        "message": "Hello World!",
    }))
}
