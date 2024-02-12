
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

use super::authentication::PublicUserData;

use crate::responses::{
    ApiResult,
    ApiResponse as ApiError, 
};

lazy_static!{

    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap()
    ;
}

type ValidationChain = HashMap<&'static str, &'static str>;

pub fn is_valid_password(password: &str) -> bool {
    let length = password.len() >= 8 && password.len() <= 30;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    
    length && has_uppercase && has_lowercase && has_digit
}

pub trait Validation {
    fn validate(&self) -> ApiResult<()>;
}

impl Validation for PublicUserData {
    
    fn validate(&self) -> ApiResult<()> {

        let mut errors: ValidationChain = HashMap::new();

        if self.name.len() < 5 || self.name.len() > 50 {
            errors.insert("name", "El nombre debe tener al menos 5 caracteres");
        }

        if self.username.len() < 5 || self.username.len() > 20 {
            errors.insert("username", "El nombre de usuario debe tener al menos 5 caracteres");
        }

        if !EMAIL_REGEX.is_match(&self.email) {
            errors.insert("email", "El email no es válido");
        }

        if !is_valid_password(&self.password) {
            errors.insert("password", "La contraseña debe tener al menos 8 caracteres");
        }

        if self.password != self.confirmPassword {
            errors.insert("confirmPassword", "Las contraseñas no coinciden");
        }

        if errors.len() > 0 {
            return Err(ApiError::BadRequest(errors))
        }

        Ok(())
    }
}
