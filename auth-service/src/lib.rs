use axum::{Router, routing::{get, post}, http::StatusCode, response::Html};
use tower_http::services::ServeDir;
use axum_server::bind;
use std::{error::Error, pin::Pin, future::Future};
use axum::response::IntoResponse;

type ServerFuture = Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send>>;

pub fn app_router() -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/verify-2fa", post(verify_mfa))
        .route("/logout", post(logout))
        .route("/verify-token", post(verify_token))
}

// This struct encapsulates our application-related logic.
pub struct Application {
    http_future: ServerFuture,
    // address is exposed as a public field,
    // so we have access to it in tests.
    pub address: String,
}

impl Application {

    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Simple handlers for the endpoints
        async fn handle_html() -> Html<&'static str> {
            Html("<html><body>Success</body></html>")
        }

        let router = app_router();

        let http_future = bind(address.parse()?)
            .serve(router.into_make_service());

        Ok(Self {
            http_future: Box::pin(http_future),
            address: format!("http://{}", address),
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.http_future.await
    }

}

async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn verify_mfa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}