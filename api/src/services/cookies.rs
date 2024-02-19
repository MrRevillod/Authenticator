
use cookie::Cookie;

pub fn new_cookie(name: &str, value: Option<&String>, type_: &str) -> Cookie<'static> {

    let exp = match type_ {

        "SESSION" => time::Duration::minutes(60),
        "REFRESH" => time::Duration::days(7),
        _ => panic!("Invalid type of cookie")
    };

    let value = match value {
        Some(value) => value.clone(),
        None => String::new()
    };

    let mut cookie = Cookie::new(name, value);

    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_path("/");
    cookie.set_max_age(exp);

    cookie.into_owned()
}