use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub target: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn parse(message: &str) -> Self {
        // TODO: Error handling
        
        let lines: Vec<&str> = message.split("\r\n").collect();
        let request_line: Vec<&str> = lines[0].split(" ").collect();

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut i: usize = 1;
        while i < lines.len() && !lines[i].is_empty() {
            let header_line = lines[i];
            if header_line.contains(": ") {
                if let Some((k, v)) = header_line.split_once(": ") {
                    // NOTE: No whitespace between key and ':'. (See standard)
                    headers.insert(k.to_string(), v.trim().to_string());
                }
            }
            i += 1;
        }

        let mut body = String::new();
        if i < lines.len() {
            body = lines[(i+1)..].join("\r\n");
        }

        Self {
            method: request_line[0].to_string(),
            target: request_line[1].to_string(),
            http_version: request_line[2].to_string(),
            headers: headers,
            body: body.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_parse_test() {
        let msg = "GET / HTTP/1.1\r\nContent-Length: 11\r\n\r\nRequesting!";
        let request = Request::parse(msg);
        assert_eq!(&request.method, "GET");
        assert_eq!(&request.target, "/");
        assert_eq!(&request.http_version, "HTTP/1.1");
        assert_eq!(
            &request.headers.get("Content-Length").unwrap().as_str(),
            &"11"
        );
        assert_eq!(&request.body, "Requesting!");
    }
}
