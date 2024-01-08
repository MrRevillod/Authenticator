
use regex::Regex;

use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::utils::types::ApiError;
use crate::models::auth_models::RegisterSchema;
use crate::models::user_models::{UpdateUserSchema, UpdateProfileSchema};

lazy_static! {

    static ref INVALID_USERNAME: String = "Username must be between 5 and 20 characters".to_string();
    static ref INVALID_EMAIL: String = "Invalid email".to_string();
    static ref INVALID_PASSWORD: String = "Password must be at least 8 characters, contain at least one uppercase letter, one lowercase letter, and one number".to_string();
    static ref PASSWORD_MISMATCH: String = "Passwords do not match".to_string();

    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
}

pub fn is_valid_password(password: &str) -> bool {
    let length = password.len() >= 8;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));

    length && has_uppercase && has_lowercase && has_digit
}

pub async fn register_data_validation(data: &RegisterSchema) -> Result<(), ApiError> {

    let mut validate_chain: HashMap<&str, String> = HashMap::new();

    if data.username.len() < 5 || data.username.len() > 20 {
        validate_chain.insert("username", INVALID_USERNAME.clone());
    }

    if !EMAIL_REGEX.is_match(&data.email) {
        validate_chain.insert("email", INVALID_EMAIL.clone());
    }

    if !is_valid_password(&data.password) {
        validate_chain.insert("password", INVALID_PASSWORD.clone());
    }

    if data.password != data.confirmPassword {
        validate_chain.insert("confirmPassword", PASSWORD_MISMATCH.clone());
    }

    if validate_chain.len() > 0 {
        return Err(ApiError::ValidationError(validate_chain));
    }

    Ok(())
}

pub async fn update_profile_validation(data: &UpdateProfileSchema) -> Result<(), ApiError> {

    let mut validate_chain: HashMap<&str, String> = HashMap::new();

    if data.username.len() < 5 || data.username.len() > 20 {
        validate_chain.insert("username", INVALID_USERNAME.clone());
    }

    if !EMAIL_REGEX.is_match(&data.email) {
        validate_chain.insert("email", INVALID_EMAIL.clone());
    }

    if !is_valid_password(&data.password) {
        validate_chain.insert("password", INVALID_PASSWORD.clone());
    }

    if data.password != data.confirmPassword {
        validate_chain.insert("confirmPassword", PASSWORD_MISMATCH.clone());
    }

    if validate_chain.len() > 0 {
        return Err(ApiError::ValidationError(validate_chain));
    }

    Ok(())
}

pub async fn update_user_validation(data: &UpdateUserSchema) -> Result<(), ApiError> {

    let mut validate_chain: HashMap<&str, String> = HashMap::new();

    if data.username.len() < 5 || data.username.len() > 20 {
        validate_chain.insert("username", INVALID_USERNAME.clone());
    }

    if !EMAIL_REGEX.is_match(&data.email) {
        validate_chain.insert("email", INVALID_EMAIL.clone());
    }

    if data.role != "ADMIN_ROLE".to_string() && data.role != "USER_ROLE".to_string() {
        validate_chain.insert("role", "Invalid role".to_string());
    }

    if data.validated != true && data.validated != false {
        validate_chain.insert("validated", "Invalid validated".to_string());
    }

    if validate_chain.len() > 0 {
        return Err(ApiError::ValidationError(validate_chain));
    }

    Ok(())
}
