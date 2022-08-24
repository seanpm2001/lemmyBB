pub mod comment;
pub mod community;
pub mod post;
pub mod site;
pub mod user;

use crate::{api::CLIENT, error::ErrorPage};
use lemmy_api_common::sensitive::Sensitive;
use rocket::http::CookieJar;

fn auth(cookies: &CookieJar<'_>) -> Option<Sensitive<String>> {
    cookies
        .get("jwt")
        .map(|c| Sensitive::new(c.value().to_string()))
}