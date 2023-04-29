use http::http_request::HttpRequest;
use http::http_response::HttpResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Handler {
    fn handle(request: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = std::env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let content = std::fs::read_to_string(full_path);
        match content {
            Ok(content) => Some(content),
            Err(_) => None,
        }
    }
}

pub struct StaticPageHandler;

pub struct PageNotFoundHandler;

pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: u32,
    order_date: String,
    order_status: String,
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = std::env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");

        let json_content = std::fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> = match json_content {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(_) => Vec::new(),
        };
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(request: &HttpRequest) -> HttpResponse {
        let http::http_request::Resource::Path(path) = &request.resource;
        let route = path.split('/').collect::<Vec<&str>>();
        // request_url: 127.0.0.1:3000/api/shipping/orders
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&WebServiceHandler::load_json()).unwrap());
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}

impl Handler for StaticPageHandler {
    fn handle(request: &HttpRequest) -> HttpResponse {
        let http::http_request::Resource::Path(path) = &request.resource;
        let route = path.split('/').collect::<Vec<&str>>();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(content) => {
                    let mut map = std::collections::HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(content))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

impl Handler for PageNotFoundHandler {
    fn handle(_request: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}