use crate::helpers::TestApp;

#[tokio::test]
async fn logins_user_success() {
    let app = TestApp::new().await;
    let email = String::from("lads@test.com");
    let password = String::from("lads123!");

    let response = app.login(email, password).await;

    assert_eq!(response.status().as_u16(), 200);
}