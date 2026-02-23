use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Input {
    text: String,
}

#[derive(Serialize)]
struct Output {
    cleaned: String,
}

async fn clean_text(Json(input): Json<Input>) -> Json<Output> {
    let prompt = format!(
        "Remove all HTML tags, footnotes, stage directions, signatures and other noise from the following text. Return only clean content, nothing else:\n\n{}",
        input.text
    );

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "qwen2.5:1.5b",
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": 0,
            }
        }))
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let cleaned = res["response"].as_str().unwrap_or("").to_string();
    Json(Output { cleaned })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/clean", post(clean_text));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
