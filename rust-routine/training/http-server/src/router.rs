use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::http_request::HttpRequest;
use std::io::Write;

pub struct Router;

impl Router {
    pub fn route(request: HttpRequest, stream: &mut impl Write) {
        match request.method {
            http::http_request::Method::GET => match &request.resource {
                http::http_request::Resource::Path(path) => {
                    let route = path.split('/').collect::<Vec<&str>>();
                    match route[1] {
                        "api" => {
                            let response = WebServiceHandler::handle(&request);
                            let _ = response.send_response(stream);
                        }
                        _ => {
                            let response = StaticPageHandler::handle(&request);
                            let _ = response.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let response = PageNotFoundHandler::handle(&request);
                let _ = response.send_response(stream);
            }
        }
    }
}
