use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:30000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("")
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("internal server error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}
