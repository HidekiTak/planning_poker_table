mod css;
mod index;
mod js;
mod not_found;
mod table;

extern crate url;

use crate::entity::Player;
use actix_http::http::header::{ETag, EntityTag};
use actix_http::http::Cookie;
use actix_http::Response;
pub use css::CssFile;
pub use index::IndexHtml;
pub use js::JsFile;
pub use not_found::NotFoundHtml;
pub use table::TableHtml;

pub struct ResponseGenerator;

impl ResponseGenerator {
    pub fn generate_response(
        if_modified_since: Option<&str>,
        etag: &str,
        content: &str,
        cookie: Option<Cookie>,
    ) -> Response {
        if let Some(since) = if_modified_since {
            if etag == since {
                return if let Some(c) = cookie {
                    Response::NotModified().cookie(c).finish()
                } else {
                    Response::NotModified().finish()
                };
            }
        }
        if let Some(c) = cookie {
            Response::Ok()
                .set(ETag(EntityTag::new(false, etag.to_string())))
                .cookie(c)
                .body(content.to_string())
        } else {
            Response::Ok()
                .set(ETag(EntityTag::new(false, etag.to_string())))
                .body(content.to_string())
        }
    }

    pub fn generate_cookie_user_name<'a>(user_name: &'a str, table_id: &'a str) -> Cookie<'a> {
        let encoded: String = url::form_urlencoded::byte_serialize(user_name.as_bytes()).collect();
        Cookie::build(Player::COOKIE_NAME, encoded)
            // .domain("localhost")
            .path(format!("/{}", table_id))
            .secure(false)
            .finish()
    }
}
