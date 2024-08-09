use std::{collections::HashMap, os::windows::process};

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Self::V1_1,
            _ => Self::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String)
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
    fn from(req: String) -> Self {
        let mut parse_method = Method::Uninitialized;
        let mut parse_version = Version::Uninitialized;
        let mut parse_resource = Resource::Path("".to_string());
        let mut parse_headers = HashMap::new();
        let mut parse_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource , version) = process_req_line(line);
                parse_method = method;
                parse_version = version;
                parse_resource = resource;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parse_headers.insert(key, value);
            } else if line.len() == 0{

            } else {
                parse_msg_body = line;
            }
        }

        HttpRequest {
            method: parse_method,
            version: parse_version,
            resource: parse_resource,
            headers: parse_headers,
            msg_body: parse_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) ->(Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap(); 

    (
        method.into(),
        Resource::Path(resource.into()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_http() {
        let s: String  = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.71.1".into());
        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers)
    }

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(Method::Get, m);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(Version::V1_1, m);
    }
}