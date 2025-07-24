use axum::{Router};
use tower_http::services::ServeDir;
use axum_server::bind;
use std::{error::Error, pin::Pin, future::Future};

type ServerFuture = Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send>>;

// This struct encapsulates our application-related logic.
pub struct Application {
    http_future: ServerFuture,
    // address is exposed as a public field,
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"));

        let http_future = bind(address.parse()?)
            .serve(router.into_make_service());

        Ok(Self {
            http_future: Box::pin(http_future),
            address: format!("https://{}", address),
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.http_future.await
    }
}