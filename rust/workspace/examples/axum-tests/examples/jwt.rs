// JWT_SECRET=secret cargo run --example jwt
// curl -s -w '\n' -H 'Content-Type: application/json' -d '{"client_id": "foo", "client_secret": "bar"}' http://localhost:3000/authorize
// {"access_token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjIwMDAwMDAwMDB9.ULPZ0NLBq9tfHroRgxJJeEYCy0tguZrEwix3fo-2dFc","token_type":"Bearer"}

use axum::{
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, encode};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "jwt=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        // .route("/protected", get(protected))
        .route("/authorize", post(authorize));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

// async fn protected(claims: Claims) -> Result<String, AuthError> {
//     Ok(format!("Welcome to the protected area :)\nYour data:\n{claims}"))
// }

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 2_000_000_000,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

#[derive(Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
