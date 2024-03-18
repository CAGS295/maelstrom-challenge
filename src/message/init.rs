use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Extra {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: u64,
    },
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::message::Body;
    use std::str::FromStr;

    #[test]
    fn init_format() {
        let obj = Body {
            msg_id: 0,
            extra: Extra::Init {
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
            extra: Extra::InitOk { in_reply_to: 0 },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":0,"type":"init_ok","in_reply_to":0}"#
        );
    }
}
