use crate::helpers::TestApp;

#[tokio::test]
async fn signup_creates_user() {
    let app = TestApp::new().await;
    let email = String::from("lads@test.com");
    let password = String::from("lads123!");
    let requires_mfa = true;

    let response = app.signup(email, password, requires_mfa).await;

    assert_eq!(response.status().as_u16(), 200);
}