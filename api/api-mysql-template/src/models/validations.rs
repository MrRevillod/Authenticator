
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::responses::{error::ApiError, ApiResult};

use super::user::{PublicUserData, UpdateUserSchema};

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
}

pub trait Validation {
    fn validate(&self) -> ApiResult<()>;
}

pub fn is_valid_password(password: &str) -> bool {
    let length = password.len() >= 8;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    
    length && has_uppercase && has_lowercase && has_digit
}

impl Validation for PublicUserData {
    
    fn validate(&self) -> ApiResult<()> {

        let mut validate_chain: HashMap<&'static str, &'static str> = HashMap::new();

        if &self.username.len() < &5 || &self.username.len() > &20 {
            validate_chain.insert("username", "Username must be between 5 and 20 characters");
        }

        if !EMAIL_REGEX.is_match(&self.email) {
            validate_chain.insert("email", "Invalid email");
        }

        if !is_valid_password(&self.password) {
            validate_chain.insert("password", "Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter and one number");
        }

        if &self.password != &self.confirmPassword {
            validate_chain.insert("confirmPassword", "Password mismatch");
        }

        if validate_chain.len() > 0 {
            return Err(ApiError::DataValidationError(validate_chain))
        }

        Ok(())
    }
}

impl Validation for UpdateUserSchema {

    fn validate(&self) -> ApiResult<()> {
        
        let mut validate_chain: HashMap<&'static str, &'static str> = HashMap::new();

        if &self.username.len() < &5 || &self.username.len() > &20 {
            validate_chain.insert("username", "Username must be between 5 and 20 characters");
        }

        if !EMAIL_REGEX.is_match(&self.email) {
            validate_chain.insert("email", "Invalid email");
        }

        if !is_valid_password(&self.password) {
            validate_chain.insert("password", "Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter and one number");
        }

        if &self.validated != &true && &self.validated != &false {
            validate_chain.insert("validated", "Account validation field must be a boolean");
        }

        if &self.role != "ADMIN_ROLE" && &self.role != "USER_ROLE" {
            validate_chain.insert("role", "User role must be User or Admin");
        }

        if validate_chain.len() > 0 {
            return Err(ApiError::DataValidationError(validate_chain))
        }

        Ok(())
    }
}
