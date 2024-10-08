use super::{Message, Reply};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo { echo: String },
    EchoOk { echo: String, in_reply_to: u64 },
}

impl<Node> Reply<Payload, &mut Node> for Message<Payload> {
    fn reply(self, _state: &mut Node) -> Option<Payload> {
        match &self.body.payload {
            Payload::Echo { echo } => Some(Payload::EchoOk {
                echo: echo.clone(),
                in_reply_to: self.body.msg_id,
            }),
            Payload::EchoOk { .. } => {
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Body;
    use super::*;
    use std::str::FromStr;

    #[test]
    fn echo_format() {
        let obj = Body {
            msg_id: 0,
            payload: Payload::Echo {
                echo: String::from_str("hola").unwrap(),
            },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":0,"type":"echo","echo":"hola"}"#
        );
    }

    #[test]
    fn echo_ok_format() {
        let obj = Body {
            msg_id: 12,
            payload: Payload::EchoOk {
                echo: String::from_str("back").unwrap(),
                in_reply_to: 1,
            },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":12,"type":"echo_ok","echo":"back","in_reply_to":1}"#
        );
    }
}
