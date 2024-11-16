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

        Self {
            method: request_line[0].to_string(),
            target: request_line[1].to_string(),
            http_version: request_line[2].to_string(),
            headers,
            body: body.to_string(),
        }
    }

    pub fn format(&self) -> String {
        let mut msg = format!("{} {} {}\r\n", 
            self.method,
            self.target, 
            self.http_version
        );

        for (key, val) in &self.headers {
            msg.push_str(&format!("{}: {}\r\n\r\n", key, val));
        }

        msg.push_str(&self.body);
        return msg;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_parse() {
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

    #[test]
    fn test_request_format() {
        let request = Request {
            method: "GET".to_string(),
            target: "/".to_string(),
            http_version: "HTTP/1.1".to_string(),
            headers: HashMap::from([
                ("Content-Length".to_string(), "11".to_string())
            ]),
            body: "Requesting!".to_string(),
        };
        let formatted = request.format();
        assert_eq!(
            &formatted,
            "GET / HTTP/1.1\r\nContent-Length: 11\r\n\r\nRequesting!"
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
