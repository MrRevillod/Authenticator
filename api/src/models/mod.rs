
pub mod user;
pub mod validations;
pub mod authentication;

use serde::Serialize;
use std::collections::HashMap;
use serde_json::{Value, to_value};

use self::user::UserProfile;

pub trait ToJson where Self: Serialize {

    fn to_json(&self) -> Value {
        to_value(self).unwrap()
    }
}

impl ToJson for UserProfile {}
impl ToJson for HashMap<String, String> {}
impl ToJson for HashMap<&'static str, &'static str> {}
