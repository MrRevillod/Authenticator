
use cookie::Cookie;

pub fn new_cookie(name: &str, value: String, exp: time::Duration) -> Cookie<'static> {
    
    let mut cookie = Cookie::new(name, value);

    cookie.set_http_only(true);
    cookie.set_secure(false);
    cookie.set_path("/");
    cookie.set_max_age(exp);

    cookie.into_owned()
}