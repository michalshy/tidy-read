use axum::{Json, Router, http::StatusCode, routing::post};
use readability_js::{Readability, ReadabilityError, ReadabilityOptions};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Input {
    html: String,
    #[serde(default)]
    url: Option<String>,
}

#[derive(Serialize)]
struct Output {
    title: Option<String>,
    author: Option<String>,
    content: String,
    text: String, // plain text
}

#[derive(Serialize)]
struct ErrorOutput {
    error: String,
}

async fn clean(Json(input): Json<Input>) -> Result<Json<Output>, (StatusCode, Json<ErrorOutput>)> {
    let reader = Readability::new().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorOutput {
                error: e.to_string(),
            }),
        )
    })?;

    let base_url = input.url.as_deref();

    let article = reader
        .parse_with_options(
            &input.html,
            base_url,
            Some(ReadabilityOptions::new().char_threshold(100)),
        )
        .map_err(|e| {
            let msg = match e {
                ReadabilityError::ReadabilityCheckFailed => {
                    "Failed to make readable text".to_string()
                }
                _ => e.to_string(),
            };
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorOutput { error: msg }),
            )
        })?;

    Ok(Json(Output {
        title: Some(article.title),
        author: article.byline,
        content: article.content,
        text: article.text_content, // plain text
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/clean", post(clean));

    println!("Server works on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
