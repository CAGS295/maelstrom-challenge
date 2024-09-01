use super::{Body, Reply};
use crate::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Broadcast {
        message: u64,
    },
    BroadcastOk {
        in_reply_to: u64,
    },
    Read,
    ReadOk {
        messages: Vec<u64>,
        in_reply_to: u64,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk {
        in_reply_to: u64,
    },
}

impl Reply<Payload, &mut Node> for Body<Payload> {
    fn reply(self, state: &mut Node) -> Payload {
        match self.payload {
            Payload::Broadcast { message } => {
                state.messages.push(message);
                Payload::BroadcastOk {
                    in_reply_to: self.msg_id,
                }
            }
            Payload::BroadcastOk { .. } => unreachable!(),
            Payload::Read => Payload::ReadOk {
                messages: state.messages.clone(),
                in_reply_to: self.msg_id,
            },
            Payload::ReadOk { .. } => unreachable!(),
            Payload::Topology { mut topology } => {
                state.neighborhood = topology
                    .remove(&state.node_id)
                    .unwrap_or_else(|| panic!("topology for {:?}", state.node_id));
                Payload::TopologyOk {
                    in_reply_to: self.msg_id,
                }
            }
            Payload::TopologyOk { .. } => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Body;
    use super::*;

    #[test]
    fn broadcast_format() {
        let obj = Body {
            msg_id: 0,
            payload: Payload::Broadcast { message: 10 },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":0,"type":"broadcast","message":10}"#
        );
    }

    #[test]
    fn broadcast_ok_format() {
        let obj = Body {
            msg_id: 12,
            payload: Payload::BroadcastOk { in_reply_to: 0 },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":12,"type":"broadcast_ok","in_reply_to":0}"#
        );
    }

    #[test]
    fn read_format() {
        let obj = Body {
            msg_id: 12,
            payload: Payload::Read,
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":12,"type":"read"}"#
        );
    }

    #[test]
    fn read_ok_format() {
        let obj = Body {
            msg_id: 12,
            payload: Payload::ReadOk {
                messages: vec![1, 2, 3],
                in_reply_to: 0,
            },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":12,"type":"read_ok","messages":[1,2,3],"in_reply_to":0}"#
        );
    }

    #[test]
    fn topology_format() {
        let mut topology = HashMap::new();
        topology.insert(format!("n0"), vec![format!("n1")]);
        let obj = Body {
            msg_id: 12,
            payload: Payload::Topology { topology },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":12,"type":"topology","topology":{"n0":["n1"]}}"#
        );
    }
}
