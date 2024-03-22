use serde::{Deserialize, Serialize};

use super::{Body, Reply};
use crate::Node;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Generate,
    GenerateOk { id: String, in_reply_to: u64 },
}

impl Reply<Payload, &mut Node> for Body<Payload> {
    fn reply(&self, state: &mut Node) -> Payload {
        match &self.payload {
            Payload::Generate => Payload::GenerateOk {
                id: format!("{}{}", state.node_id, state.msg_id),
                in_reply_to: self.msg_id,
            },
            Payload::GenerateOk { .. } => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Body;
    use super::*;

    #[test]
    fn generate_format() {
        let obj = Body {
            msg_id: 0,
            payload: Payload::Generate,
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":0,"type":"generate"}"#
        );
    }

    #[test]
    fn generate_ok_format() {
        let obj = Body {
            msg_id: 12,
            payload: Payload::GenerateOk {
                id: format!("2"),
                in_reply_to: 1,
            },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":12,"type":"generate_ok","id":"2","in_reply_to":1}"#
        );
    }
}
