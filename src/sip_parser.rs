use std::collections::HashMap;

pub struct SipMessage {
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub message_type: String,
    pub extra: Option<String>,
}
impl SipMessage {
    pub fn new(raw_message: &str) -> Self {
        let headers_body_split = raw_message.split("\r\n\r\n").collect::<Vec<&str>>();
        let mut headers_split = headers_body_split[0].split("\r\n").collect::<Vec<&str>>();
        let message_type = headers_split
            .get(0)
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()[0];
        let extra = headers_body_split.get(1).map(|s| s.to_string());
        headers_split.remove(0);
        let mut headers = HashMap::new();
        for header in headers_split {
            let header_kv_split = header.split(": ").collect::<Vec<&str>>();
            headers.insert(
                header_kv_split[0].to_string(),
                header_kv_split[1].to_string(),
            );
        }
        let body = headers_body_split.get(1).map(|s| s.to_string());
        Self {
            message_type: message_type.to_string(),
            headers,
            body,
            extra,
        }
    }
}
