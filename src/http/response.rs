use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub http_version: String,
    pub status_code: String,
    pub reason_phrase: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn parse(message: &str) -> Self {
        // TODO: Error handling
        
        let lines: Vec<&str> = message.split("\r\n").collect();
        let status_line: Vec<&str> = lines[0].split(" ").collect();
        // NOTE: Reason optional, but must have space after code (See standard)

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut i: usize = 1;
        while i < lines.len() && !lines[i].is_empty() {
            let header_line = lines[i];
            if header_line.contains(": ") {
                // NOTE: No whitespace between key and ':'. (See standard)
                if let Some((k, v)) = header_line.split_once(": ") {
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
            http_version: status_line[0].to_string(),
            status_code: status_line[1].to_string(),
            reason_phrase: status_line[2].to_string(),
            headers: headers,
            body: body.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_parse() {
        let msg = "HTTP/1.1 200 OK\r\nContent-Length: 19\r\n\r\nResponded!";
        let response = Response::parse(msg);
        assert_eq!(&response.http_version, "HTTP/1.1");
        assert_eq!(&response.status_code, "200");
        assert_eq!(&response.reason_phrase, "OK");
        assert_eq!(
            &response.headers.get("Content-Length").unwrap().as_str(),
            &"19"
        );
        assert_eq!(&response.body, "Responded!");
    }
}