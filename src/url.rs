#[derive(Default, Debug)]
pub struct Url {
    scheme: String,
    netloc: String,
    path: String,
    params: String,
    query: String,
    fragment: String,
    hostname: String,
    port: String,
}

impl Url {
    pub fn get_url(&self) -> String {
        let mut url = String::new();

        if !self.scheme.is_empty() {
            url.push_str(&self.scheme);
            url.push_str("://");
        }

        url.push_str(&self.netloc);
        url.push_str(&self.path);

        if !self.query.is_empty() {
            url.push_str("?");
            url.push_str(&self.query);
        }

        if !self.params.is_empty() {
            url.push_str(";");
            url.push_str(&self.params);
        }

        if !self.fragment.is_empty() {
            url.push_str("#");
            url.push_str(&self.fragment);
        }
        url
    }

    pub fn parse(url: &str) -> Self {
        let mut result = Url::default();
        let mut remaining_url = url;

        if let Some((scheme, rest)) = remaining_url.split_once("://") {
            result.scheme = scheme.to_string();     
            remaining_url = rest;
        }

        if let Some((rest, fragment)) = remaining_url.split_once("#") {
            result.fragment = fragment.to_string();
            remaining_url = rest;
        }

        if let Some((rest, query)) = remaining_url.split_once("?") {
            result.query= query.to_string();
            remaining_url = rest;
        }

        if let Some((rest, params)) = remaining_url.split_once(";") {
            result.params = params.to_string();
            remaining_url = rest;
        }

        if let Some((rest, path)) = remaining_url.split_once("/") {
            result.path = format!("/{}", path);
            remaining_url = rest;
        } 

        if let Some((hostname, port)) = remaining_url.split_once(":") {
            result.hostname = hostname.to_string();     
            result.port = port.to_string();     
            result.netloc = remaining_url.to_string();
        } else {
            result.hostname = remaining_url.to_string();
            result.netloc = remaining_url.to_string();
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
