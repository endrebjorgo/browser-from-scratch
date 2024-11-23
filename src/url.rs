#[derive(Default, Debug)]
pub struct Url {
    scheme: String,
    netloc: String,
    hostname: String,
    port: String,
    path: String,
    query: String,
    fragment: String,
}

impl Url {
    pub fn parse(url: &str) -> Self {
        let mut result = Url::default();
        let mut remaining_url = url;

        if let Some((scheme, rest)) = remaining_url.split_once("://") {
            result.scheme = scheme.to_string();     
            remaining_url = rest;
        }

        if let Some((netloc, rest)) = remaining_url.split_once("/") {
            result.netloc = netloc.to_string();     
            remaining_url = rest;
        }

        if let Some((hostname, port)) = result.netloc.split_once(":") {
            result.hostname = hostname.to_string();     
            result.port = port.to_string();     
        } else {
            result.hostname = result.netloc;     
            result.port = match result.scheme.as_str() {
                "https" => "443".to_string(),
                "http" => "80".to_string(),
                _ => unimplemented!(),
            };
            result.netloc = format!("{}:{}", result.hostname, result.port);
        }

        if let Some((path, rest)) = remaining_url.split_once("?") {
            result.path = format!("/{}", path);
            remaining_url = rest;
        }

        if let Some((query, rest)) = remaining_url.split_once("#") {
            result.query= query.to_string();
            remaining_url = rest;
        }

        if !remaining_url.is_empty() {
            result.fragment = remaining_url.to_string();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_parse() {
        let url = Url::parse("http://docs.python.org:80/3/library/urllib.parse.html?highlight=params#url-parsing");
        assert_eq!(&url.scheme, "http");
        assert_eq!(&url.netloc, "docs.python.org:80");
        assert_eq!(&url.hostname, "docs.python.org");
        assert_eq!(&url.port, "80");
        assert_eq!(&url.path, "/3/library/urllib.parse.html");
        assert_eq!(&url.query, "highlight=params");
        assert_eq!(&url.fragment, "url-parsing");
    }
}
