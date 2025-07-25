use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use crate::dtos::SignupRequestBody;

pub async fn signup(Json(request): Json<SignupRequestBody>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}
