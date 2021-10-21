mod planning_poker;

use crate::resource::ResponseGenerator;
use actix_http::Response;
use planning_poker::PlanningPokerJs;

pub struct JsFile;

impl JsFile {
    pub fn get(name: &str, if_modified_since: Option<&str>) -> Option<Response> {
        match name {
            "planning_poker.js" => Some(ResponseGenerator::generate_response(
                if_modified_since,
                PlanningPokerJs::ETAG,
                PlanningPokerJs::CONTENT,
                None,
            )),
            _ => None,
        }
    }
}
