mod planning_poker;

use crate::resource::ResponseGenerator;
use actix_http::Response;
use planning_poker::PlanningPokerCss;

pub struct CssFile;

impl CssFile {
    pub fn get(name: &str, if_modified_since: Option<String>) -> Option<Response> {
        match name {
            "planning_poker.css" => Some(ResponseGenerator::generate_response(
                if_modified_since,
                PlanningPokerCss::ETAG,
                PlanningPokerCss::CONTENT,
                None,
            )),
            _ => None,
        }
    }
}
