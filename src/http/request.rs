use std::io::{Read, Write, ErrorKind};
use std::net::TcpStream;
use std::collections::HashMap;
use crate::http::response::Response;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST=> "POST",
        };
        write!(f, "{}", value)
    }
}

impl std::str::FromStr for HttpMethod {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => Err(format!("'{}' is not a valid HttpMethod", s)),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub target: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    fn new(method: HttpMethod) -> Self {
        Self {
            method,
            target: "/".to_string(),
            http_version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn get(url: &str) -> Self {
        Self::new(HttpMethod::GET)
            .add_header("Host", url)
            .add_header("Connection", "close")
    }

    pub fn set_target(mut self, target: &str) -> Self {
        self.target = target.to_string();
        self
    }

    pub fn add_header(mut self, key: &str, val: &str) -> Self {
        self.headers.insert(key.to_string(), val.to_string());
        self
    }

    pub fn set_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn parse(message: &str) -> Self {
        // TODO: Error handling
        let lines: Vec<&str> = message.split("\r\n").collect();
        let request_line: Vec<&str> = lines[0].split(" ").collect();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut i: usize = 1;

        while i < lines.len() && !lines[i].is_empty() {
            let header_line = lines[i];
            if header_line.contains(": ") {
                if let Some((key, val)) = header_line.split_once(": ") {
                    // NOTE: No whitespace between key and ':'. (See standard)
                    headers.insert(key.to_string(), val.trim().to_string());
                }
            }
            i += 1;
        }

        let mut body = String::new();
        if i < lines.len() {
            body = lines[(i+1)..].join("\r\n");
        }

        let method = request_line[0].parse::<HttpMethod>().unwrap();
        
        Self {
            method,
            target: request_line[1].to_string(),
            http_version: request_line[2].to_string(),
            headers,
            body: body.to_string(),
        }
    }

    pub fn format(&self) -> String {
        let mut msg = format!("{} {} {}\r\n", 
            self.method.to_string(),
            self.target, 
            self.http_version
        );

        for (key, val) in &self.headers {
            msg.push_str(&format!("{}: {}\r\n", key, val));
        }

        msg.push_str("\r\n");
        msg.push_str(&self.body);
        return msg;
    }

    pub fn send(&self) -> Result<Response, std::io::Error>  {
        let mut url = String::new();
        if let Some(val) = self.headers.get("Host") {
            url.push_str(val);
            url.push_str(":80");
        } else {
            return Err(std::io::Error::new(ErrorKind::InvalidInput, "No host"));
        }

        let mut stream = TcpStream::connect(url)?;
        let request = self.format();
        stream.write_all(request.as_bytes())?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        Ok(Response::parse(&response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_parse() {
        let msg = "GET / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 11\r\n\r\nRequesting!";
        let request = Request::parse(msg);
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(&request.target, "/");
        assert_eq!(&request.http_version, "HTTP/1.1");
        assert_eq!(
            &request.headers.get("Host").unwrap().as_str(),
            &"example.com"
        );
        assert_eq!(
            &request.headers.get("Content-Length").unwrap().as_str(),
            &"11"
        );
        assert_eq!(&request.body, "Requesting!");
    }

    #[test]
    fn test_request_format() {
        let request = Request {
            method: HttpMethod::GET,
            target: "/".to_string(),
            http_version: "HTTP/1.1".to_string(),
            headers: HashMap::from([
                ("Host".to_string(), "example.com".to_string()),
                ("Content-Length".to_string(), "11".to_string()),
            ]),
            body: "Requesting!".to_string(),
        };
        let formatted = request.format();
        // NOTE: Ordering of headers are arbitrary. Must find some solution.
        assert!(
            &formatted == "GET / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 11\r\n\r\nRequesting!" ||
            &formatted == "GET / HTTP/1.1\r\nContent-Length: 11\r\nHost: example.com\r\n\r\nRequesting!"
        );
    }

    #[test]
    fn test_request_parse_then_format() {
        let msg = "GET / HTTP/1.1\r\nContent-Length: 11\r\n\r\nRequesting!";
        let parsed = Request::parse(msg);
        let formatted = parsed.format();
        assert_eq!(msg, &formatted);
    }
}
