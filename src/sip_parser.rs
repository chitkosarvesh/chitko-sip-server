use std::collections::HashMap;

pub enum SipMessageType {
    REGISTER,
    INVITE,
    ACK,
    UNKNOWN,
}
pub struct SipMessage {
    pub message_type: SipMessageType,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl SipMessage {
    pub fn new(msg_string: String) -> Self {
        let mut parsed_message = Self {
            message_type: SipMessageType::UNKNOWN,
            body: "".to_string(),
            headers: HashMap::new(),
        };
        let header_body_split = msg_string.split("\r\n\r\n").collect::<Vec<&str>>();
        let headers = header_body_split[0].split("\r\n").collect::<Vec<&str>>();
        match headers[0].split(" ").collect::<Vec<&str>>()[0] {
            "REGISTER" => parsed_message.message_type = SipMessageType::REGISTER,
            "INVITE" => parsed_message.message_type = SipMessageType::INVITE,
            "ACK" => parsed_message.message_type = SipMessageType::ACK,
            _ => parsed_message.message_type = SipMessageType::UNKNOWN,
        }
        for (i, header) in headers.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let header_split = header.split(": ").collect::<Vec<&str>>();
            parsed_message
                .headers
                .insert(header_split[0].to_string(), header_split[1].to_string());
        }
        parsed_message.body = header_body_split[1].to_string();
        parsed_message
    }
}
