mod entity;
mod page;
mod resource;
mod web_socket_session;

use crate::entity::{Id, Player, TableContainer};
use crate::page::AttendHtml;
use crate::resource::{CssFile, IndexHtml, JsFile, NotFoundHtml, ResponseGenerator, TableHtml};
use actix_web::http::header;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;

// <editor-fold desc="pages">

fn get_if_none_match(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get(header::IF_NONE_MATCH)?
        .to_str()
        .ok()
        .map(|str| {
            if str.as_bytes()[0] == b'"' {
                str[1..(str.len() - 1)].to_string()
            } else {
                str.to_string()
            }
        })
}

/// Table開設ページ
#[get("/")]
async fn index(req: HttpRequest, _path: web::Path<()>) -> impl Responder {
    // async fn handler(params: web::Path<(String, String)>) {
    // let (foo, bar) = params.into_inner();
    let lang: &str = req
        .headers()
        .get(actix_web::http::header::ACCEPT_LANGUAGE)
        .map(|l| l.to_str().unwrap_or_default())
        .unwrap_or_default();
    let now: i64 = Utc::now().timestamp_millis();
    if lang.is_empty() {
        let peer_addr = req.peer_addr().map(|addr| addr.to_string());
        println!(
            "{}",
            json!({
                "comand": "index",
                "peer_addr": peer_addr,
                "at": now
            })
        );
    } else {
        println!(r#"{{"comand":"index","langs":"{}","at":{}}}"#, lang, now);
    }

    let index_html: IndexHtml = IndexHtml::instance();
    ResponseGenerator::generate_response(
        get_if_none_match(&req),
        index_html.etag(),
        index_html.content(),
        None,
    )
}

#[derive(Serialize, Deserialize)]
pub struct FormParams {
    table: Option<String>,
    name: String,
    sel_opt: Option<String>,
    sel_val: Option<String>,
}

/// Table開設実行
/// Tableを作って、FormParamsを持ったままtableにPost
#[post("/")]
async fn new_table(params: web::Form<FormParams>) -> impl Responder {
    let table_name: Option<&str> = params.table.as_deref();
    let options: Option<Vec<String>> =
        split_map(&params.sel_opt).or_else(|| split_map(&params.sel_val));

    let table_id: String = TableContainer::instance().preserve(table_name, options);
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, format!("/{}", table_id)))
        .finish()
}

fn split_map(str: &Option<String>) -> Option<Vec<String>> {
    str.clone()
        .map(|s| split(s.as_str()))
        .filter(|v| !v.is_empty())
}

fn split(str: &str) -> Vec<String> {
    str.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(NotFoundHtml::CONTENT)
}

#[get("/check.html")]
async fn check() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

///
/// Player参加用ページ
#[get("/{table_id}")]
async fn table(path: web::Path<String>) -> impl Responder {
    let table_id: String = path.into_inner();
    match TableContainer::instance().status_of(table_id.as_str()) {
        Ok(r) => HttpResponse::Ok()
            .content_type("text/html")
            .body(AttendHtml::content(r.table_name())),
        Err(_) => HttpResponse::NotFound()
            .content_type("text/html")
            .body(NotFoundHtml::CONTENT),
    }
}

///
/// Tableの
#[post("/{table_id}")]
async fn new_player(
    req: HttpRequest,
    path: web::Path<String>,
    params: web::Form<FormParams>,
) -> impl Responder {
    let table_id: String = path.into_inner();
    match TableContainer::instance().status_of(table_id.as_str()) {
        Ok(r) => {
            if r.player_count() < &16 {
                let cookie = Some(ResponseGenerator::generate_cookie_user_name(
                    params.name.as_str(),
                    table_id.as_str(),
                ));
                ResponseGenerator::generate_response(
                    get_if_none_match(&req),
                    TableHtml::ETAG,
                    TableHtml::CONTENT,
                    cookie,
                )
            } else {
                HttpResponse::Ok()
                    .content_type("text/html")
                    .body(NotFoundHtml::CONTENT)
            }
        }
        Err(_) => HttpResponse::NotFound()
            .content_type("text/html")
            .body(NotFoundHtml::CONTENT),
    }
}

// </editor-fold>

// -------------------------------------------------------------------------------------------------
//
//  WebSocket
//

#[get("/{table_id}/ws")]
async fn ws_entry(
    req: HttpRequest,
    path: web::Path<String>,
    stream: web::Payload,
) -> Result<HttpResponse, actix_http::Error> {
    let table_id: String = path.into_inner();
    let name: String = req
        .cookie(Player::COOKIE_NAME)
        .map(|x| x.value().to_string())
        .filter(|x| !x.is_empty())
        .map(|n| {
            url::form_urlencoded::parse(n.as_bytes())
                .map(|(key, val)| [key, val].concat())
                .collect()
        })
        .unwrap_or_else(|| Id::generate("", None)[..8].to_string());
    TableContainer::instance()
        .start_web_socket(req, name, table_id.as_str(), stream)
        .await
}

// <editor-fold desc="resource">

// -------------------------------------------------------------------------------------------------
//
//  Resources
//

#[get("/js/{file_name}")]
async fn js(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let file_name: String = path.into_inner();
    JsFile::get(file_name.as_str(), get_if_none_match(&req))
        .unwrap_or_else(|| HttpResponse::NotFound().finish())
}

#[get("/css/{file_name}")]
async fn css(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let file_name: String = path.into_inner();
    CssFile::get(file_name.as_str(), get_if_none_match(&req))
        .unwrap_or_else(|| HttpResponse::NotFound().finish())
}
// </editor-fold>

// -------------------------------------------------------------------------------------------------
//
//  Main
//

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(check)
            .service(favicon)
            .service(index)
            .service(new_table)
            .service(table)
            .service(new_player)
            .service(css)
            .service(js)
            .service(ws_entry)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {
        println!("test")
    }
}
