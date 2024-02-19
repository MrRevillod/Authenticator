
use serde_json::json;
use lazy_static::lazy_static;

use super::reqwest::http_request;
use crate::responses::{Response, ApiResult};

use crate::config::state::*;

lazy_static! {
    static ref RECOVER_PASSWORD_ENDPOINT: String = format!("{}/forgot-password-email", *MAILER_SERVICE_URL);
    static ref VALIDATE_ACC_ENDPOINT: String = format!("{}/email-verification", *MAILER_SERVICE_URL);
    static ref CONFIRM_NEW_EMAIL_ENDPOINT: String = format!("{}/email-change", *MAILER_SERVICE_URL);
}

/// Sends an email to the user with a link to reset their password

pub async fn send_recovery_email(
    email: &String, url: &String) -> ApiResult<()> {

    let body = json!({ "email": email, "url": url });
    let response = http_request(&RECOVER_PASSWORD_ENDPOINT, "POST", body).await;

    match response.status().as_u16() {
        200 => (),
        400 => return Err(Response::BAD_REQUEST),
        500 => return Err(Response::INTERNAL_SERVER_ERROR),
        _ => return Err(Response::INTERNAL_SERVER_ERROR)
    }
    
    Ok(())
}

/// Sends an email to the user with a link to validate their account

pub async fn send_acc_validation_email(
    email: &String, url: &String) -> ApiResult<()> {

    let body = json!({ "email": email, "url": url });
    let response = http_request(&VALIDATE_ACC_ENDPOINT, "POST", body).await;

    match response.status().as_u16() {
        200 => (),
        400 => return Err(Response::BAD_REQUEST),
        _ => return Err(Response::INTERNAL_SERVER_ERROR)
    }

    Ok(())
}

pub async fn send_new_email_confirmation(
    email: &String, url: &String) -> ApiResult<()> {

    let body = json!({ "email": email, "url": url });
    let response = http_request(&CONFIRM_NEW_EMAIL_ENDPOINT, "POST", body).await;

    match response.status().as_u16() {
        200 => (),
        400 => return Err(Response::BAD_REQUEST),
        _ => return Err(Response::INTERNAL_SERVER_ERROR)
    }

    Ok(())
}
