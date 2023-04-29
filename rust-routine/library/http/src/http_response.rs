use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    pub version: &'a str,
    pub status_code: &'a str,
    pub status_text: &'a str,
    pub headers: Option<HashMap<&'a str, &'a str>>,
    pub body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(value: HttpResponse<'a>) -> Self {
        let res = value.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res.version(),
            &res.status_code(),
            &res.status_text(),
            &res.headers(),
            &value.body.unwrap().len(),
            &res.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        msg_body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = msg_body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl std::io::Write) -> std::io::Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut headers_string = String::new();
        for (key, value) in map.iter() {
            headers_string = format!("{}{}:{}\r\n", headers_string, key, value);
        }
        headers_string
    }

    fn body(&self) -> &str {
        match &self.body {
            Some(body) => body.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_200() {
        let response = HttpResponse::new("200", None, Some("Hello World!".to_string()));
        let expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: Some({
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                h
            }),
            body: Some("Hello World!".to_string()),
        };
        assert_eq!(response, expected);
    }

    #[test]
    fn test_response_404() {
        let response = HttpResponse::new("404", None, Some("Not Found".to_string()));
        let expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: Some({
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                h
            }),
            body: Some("Not Found".to_string()),
        };
        assert_eq!(response, expected);
    }

    #[test]
    fn test_response_string() {
        let response = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: Some({
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                h
            }),
            body: Some("Hello World!".to_string()),
        };
        let response_string = String::from(response);
        let expected =
            "HTTP/1.1 200 OK\r\nContent-Type:text/html\r\nContent-Length: 12\r\n\r\nHello World!";
        assert_eq!(response_string, expected);
    }
}
