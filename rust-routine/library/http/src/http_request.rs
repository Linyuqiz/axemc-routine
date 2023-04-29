use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    HTTP1_1,
    HTTP2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::HTTP1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(value: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::HTTP1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in value.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_request_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_msg_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_header_line(s: &str) -> (String, String) {
    let mut head_items = s.split(":");
    let key = head_items.next().unwrap().trim().to_string();
    let value = head_items.next().unwrap().trim().to_string();
    (key, value)
}

fn process_request_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = Method::from(words.next().unwrap());
    let resource = Resource::Path(words.next().unwrap().to_string());
    let version = Version::from(words.next().unwrap());
    (method, resource, version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(Method::from("GET"), Method::GET);
        assert_eq!(Method::from("POST"), Method::POST);
        assert_eq!(Method::from("PUT"), Method::Uninitialized);
    }

    #[test]
    fn test_version_from() {
        assert_eq!(Version::from("HTTP/1.1"), Version::HTTP1_1);
        assert_eq!(Version::from("HTTP/2.0"), Version::Uninitialized);
    }

    #[test]
    fn test_read_request() {
        let request_str: String = String::from("GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "localhost".into());
        headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());
        headers_expected.insert("Accept".into(), "*/*".into());

        let request: HttpRequest = request_str.into();
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.version, Version::HTTP1_1);
        assert_eq!(request.resource, Resource::Path("/".to_string()));
        assert_eq!(request.headers, headers_expected);
    }
}
