use super::{Body, Reply};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename = "init")]
#[serde(tag = "type")]
pub struct Init {
    pub node_id: String,
    node_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename = "init_ok")]
pub struct InitOk {
    pub in_reply_to: u64,
}

impl Reply<InitOk> for Body<Init> {
    fn reply(&self) -> InitOk {
        InitOk {
            in_reply_to: self.msg_id,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::message::Body;
    use std::str::FromStr;

    #[test]
    fn init_format() {
        let obj = Body {
            msg_id: 0,
            payload: Init {
                node_id: String::from_str("n0").unwrap(),
                node_ids: vec![String::from_str("n0").unwrap()],
            },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":0,"type":"init","node_id":"n0","node_ids":["n0"]}"#
        );
    }

    #[test]
    fn init_ok_format() {
        let obj = Body {
            msg_id: 0,
            payload: InitOk { in_reply_to: 0 },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":0,"type":"init_ok","in_reply_to":0}"#
        );
    }
}
