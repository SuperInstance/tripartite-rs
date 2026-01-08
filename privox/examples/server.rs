//! HTTP redaction server example
//!
//! This example demonstrates running an HTTP API for redaction
//! using the warp web framework.
//!
//! Run with:
//!   cargo run --example server
//!
//! Test with:
//!   curl -X POST http://localhost:3030/redact \
//!     -H "Content-Type: application/json" \
//!     -d '{"text": "Email: john@example.com", "session_id": "test"}'

use privox::{Redactor, RedactorConfig, TokenVault};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
use warp::Filter;

#[derive(Deserialize)]
struct RedactRequest {
    text: String,
    session_id: String,
}

#[derive(Serialize)]
struct RedactResponse {
    redacted_text: String,
    #[serde(flatten)]
    stats: privox::RedactionStats,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shared redactor (Arc+Mutex for thread-safe access)
    let vault = TokenVault::in_memory()?;
    let redactor = Arc::new(Mutex::new(Redactor::new(RedactorConfig::default(), vault)?));

    // Health check endpoint
    let health =
        warp::path("health").map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    // Redact endpoint
    let redact_route = warp::post()
        .and(warp::path("redact"))
        .and(warp::body::json())
        .and(warp::any().map(move || redactor.clone()))
        .map(|req: RedactRequest, redactor: Arc<Mutex<Redactor>>| {
            // Lock and redact
            let mut r = match redactor.lock() {
                Ok(guard) => guard,
                Err(e) => {
                    return warp::reply::with_status(
                        warp::reply::json(&ErrorResponse {
                            error: format!("Mutex poisoned: {}", e),
                        }),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                },
            };

            match r.redact(&req.text, &req.session_id) {
                Ok(result) => warp::reply::with_status(
                    warp::reply::json(&RedactResponse {
                        redacted_text: result.redacted_text,
                        stats: result.stats,
                    }),
                    warp::http::StatusCode::OK,
                ),
                Err(e) => warp::reply::with_status(
                    warp::reply::json(&ErrorResponse {
                        error: e.to_string(),
                    }),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            }
        });

    // Reinflate endpoint
    #[derive(Deserialize)]
    struct ReinflateRequest {
        text: String,
    }

    let reinflate_route = warp::post()
        .and(warp::path("reinflate"))
        .and(warp::body::json())
        .and(warp::any().map(move || redactor.clone()))
        .map(|req: ReinflateRequest, redactor: Arc<Mutex<Redactor>>| {
            let r = match redactor.lock() {
                Ok(guard) => guard,
                Err(e) => {
                    return warp::reply::with_status(
                        warp::reply::json(&ErrorResponse {
                            error: format!("Mutex poisoned: {}", e),
                        }),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                },
            };

            let restored = r.reinflate(&req.text);
            warp::reply::json(&serde_json::json!({
                "restored_text": restored
            }))
        });

    // Combine routes
    let routes = health
        .or(redact_route)
        .or(reinflate_route)
        .with(warp::cors().allow_any_origin());

    println!("🚀 Redaction server starting on http://127.0.0.1:3030");
    println!();
    println!("Endpoints:");
    println!("  POST /redact    - Redact sensitive data");
    println!("  POST /reinflate - Restore original values");
    println!("  GET  /health    - Health check");
    println!();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
