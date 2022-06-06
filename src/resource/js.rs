mod index;
mod planning_poker;

use crate::resource::ResponseGenerator;
use actix_web::HttpResponse;
use index::IndexJs;
use planning_poker::PlanningPokerJs;

pub struct JsFile;

impl JsFile {
    pub fn get(name: &str, if_modified_since: Option<String>) -> Option<HttpResponse> {
        match name {
            "planning_poker.js" => Some(ResponseGenerator::generate_response(
                if_modified_since,
                PlanningPokerJs::ETAG,
                PlanningPokerJs::CONTENT,
                None,
            )),
            "index.js" => Some(ResponseGenerator::generate_response(
                if_modified_since,
                IndexJs::ETAG,
                IndexJs::CONTENT,
                None,
            )),
            _ => None,
        }
    }
}
