mod css;
mod index;
mod js;
mod not_found;
mod table;

extern crate url;

use crate::entity::Player;
use actix_web::cookie::Cookie;
use actix_web::http::header::{ETag, EntityTag};
use actix_web::HttpResponse;
pub use css::CssFile;
pub use index::IndexHtml;
pub use js::JsFile;
pub use not_found::NotFoundHtml;
pub use table::TableHtml;

pub struct ResponseGenerator;

impl ResponseGenerator {
    pub fn generate_response(
        if_modified_since: Option<String>,
        etag: &str,
        content: &str,
        cookie: Option<Cookie>,
    ) -> HttpResponse {
        if let Some(since) = if_modified_since {
            if etag == since {
                return if let Some(c) = cookie {
                    HttpResponse::NotModified().cookie(c).finish() // ::NotModified().cookie(c).finish()
                } else {
                    HttpResponse::NotModified().finish()
                };
            }
        }
        if let Some(c) = cookie {
            HttpResponse::Ok()
                .cookie(c)
                .insert_header(ETag(EntityTag::new(false, etag.to_string())))
                .body(content.to_string())
        } else {
            HttpResponse::Ok()
                .insert_header(ETag(EntityTag::new(false, etag.to_string())))
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
