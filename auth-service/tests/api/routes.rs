use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup_creates_user() {
    let app = TestApp::new().await;
    let email = String::from("lads@test.com");
    let password = String::from("lads123!");
    let requires_mfa = true;

    let response = app.signup(email, password, requires_mfa).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logins_user_success() {
    let app = TestApp::new().await;
    let email = String::from("lads@test.com");
    let password = String::from("lads123!");

    let response = app.login(email, password).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_user_success() {
    let app = TestApp::new().await;
    let jwt = String::from("jwt");
    let response = app.logout(jwt).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_success() {
    let app = TestApp::new().await;
    let jwt = String::from("jwt");
    let response = app.verify_token(jwt).await;

    assert_eq!(response.status().as_u16(), 200);
}