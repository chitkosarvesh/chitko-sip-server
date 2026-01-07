use std::collections::HashMap;

/// Types of valid SIP messages
pub enum SipMessageType {
    REGISTER,
    INVITE,
    UNKNOWN,
}

/// implements conversion from string to enum and vice versa
impl SipMessageType {
    pub fn from_str(message_type: &str) -> Self {
        match message_type {
            "REGISTER" => Self::REGISTER,
            "INVITE" => Self::INVITE,
            _ => Self::UNKNOWN,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::REGISTER => "REGISTER",
            Self::INVITE => "INVITE",
            Self::UNKNOWN => "UNKNOWN",
        }
    }
}

/// struct to hold SIP message data
pub struct SipMessage {
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub message_type: SipMessageType,
    extra: Option<String>,
}

/// implements the creation of a SipMessage from a raw SIP message string
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
            message_type: SipMessageType::from_str(message_type),
            headers,
            body,
            extra,
        }
    }
}

/// struct to hold SIP response data
pub struct SipResponse {
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

/// implements the creation of a SipResponse from its components
impl SipResponse {
    pub fn new(
        status_code: u16,
        reason_phrase: String,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Self {
        Self {
            status_code,
            reason_phrase,
            headers: match headers {
                None => HashMap::new(),
                Some(h) => h,
            },
            body,
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let string = self.as_string();
        string.into_bytes()
    }
    pub fn as_string(&self) -> String {
        let mut response_string = String::new();
        response_string.push_str(&format!(
            "SIP/2.0 {} {}\r\n",
            self.status_code, self.reason_phrase
        ));
        for (key, value) in &self.headers {
            response_string.push_str(&format!("{}: {}\r\n", key, value));
        }
        response_string.push_str("\r\n");
        if let Some(body) = &self.body {
            response_string.push_str(body);
        }
        response_string
    }
}

/// handles REGISTER requests
pub fn handle_register(message: SipMessage) -> Option<SipResponse> {
    log::info!("Received REGISTER request from {}", message.headers["From"]);
    // check if the authorization header exists, if not, send 401
    if message.headers.get("Authorization") != None {
        None
    } else {
        Some(create_unauthorized_response(message))
    }
}

fn create_unauthorized_response(message: SipMessage) -> SipResponse {
    SipResponse::new(401, "Unauthorized".to_string(), Some(message.headers), None)
}
