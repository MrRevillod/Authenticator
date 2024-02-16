
use reqwest::Body;
use serde_json::{json, to_vec};

use crate::{
    
    responses::{
        Response,
        ApiResult,
    },
    
    config::state::*,
};

pub async fn reset_password_email_service(email: &String, url: &String) -> ApiResult<()> {

    let client = reqwest::Client::new();
    let body = json!({ "email": email, "url": url });
    let body_bytes = to_vec(&body)
        .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    let mailer_url = format!("{}/forgot-password-email", MAILER_SERVICE_URL.to_string());

    let response = client
        .post(mailer_url)
        .header("Content-Type", "application/json")
        .header("x-api-key", MAILER_API_KEY.to_string())
        .body(Body::from(body_bytes))
        .send()
        .await
        .map_err(|_| Response::INTERNAL_SERVER_ERROR)?
    ;

    match response.status().as_u16() {
        200 => (),
        400 => return Err(Response::BAD_REQUEST),
        _ => return Err(Response::INTERNAL_SERVER_ERROR)
    }

    Ok(())
}