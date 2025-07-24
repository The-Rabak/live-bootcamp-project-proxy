use reqwest::{Error, Response};
use auth_service::Application;
use serde::{Serialize, Serializer};
use serde::ser::{SerializeStruct, SerializeStructVariant};

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

pub struct SignupBody {
    pub email: String,
    pub password: String,
    pub requires_mfa: bool
}

impl Serialize for SignupBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("SignupBody", 3)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("password", &self.password)?;
        state.serialize_field("requires2FA", &self.requires_mfa)?;
        state.end()
    }
}

#[derive(Serialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String
}
pub struct Verify2FABody {
    pub email: String,
    pub login_attempt_id: String,
    pub mfa_code: String,
}

impl Serialize for Verify2FABody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("Verify2FABody", 3)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("loginAttemptId", &self.login_attempt_id)?;
        state.serialize_field("2FACode", &self.mfa_code)?;
        state.end()
    }
}

#[derive(Serialize)]
pub struct VerifyJWTBody {
    pub token: String
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        Self { address, http_client }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn signup(&self, email: String, password: String, requires_mfa: bool) -> Result<Response, Error> {
        let body = SignupBody {
            email,
            password,
            requires_mfa,
        };

        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(&body)
            .header("Content-Type", "application/json")
            .send()
            .await
    }

    pub async fn login(&self, email: String, password: String) -> Result<Response, Error> {
        let body = LoginBody {
            email,
            password
        };

        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(&body)
            .header("Content-Type", "application/json")
            .send()
            .await
    }

    pub async fn logout(&self, jwt: String) -> Result<Response, Error> {

        self.http_client
            .post(&format!("{}/logout", &self.address))
            .json(&body)
            .header("Content-Type", "application/json")
            .header("Cookie", format!("jwt={}", jwt))
            .send()
            .await
    }

    pub async fn verify_token(&self, jwt_token: String) -> Result<Response, Error> {
        let body = VerifyJWTBody {
            token: jwt_token
        };

        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(&body)
            .header("Content-Type", "application/json")
            .send()
            .await
    }


}
