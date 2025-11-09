use super::{Message, Reply};
use crate::Node;
use bloomfilter::Bloom;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

pub mod bloom_serde;

/*
    grid-top; max. prop. time ∝ (latency + gossip) * sqrt(n) hops
    let n: 25; max prop time: 2s; latency: 100ms

    optimal gossip: 2_000ms = (100 + G) * sqrt(25)
    G = 2_000 / 5 - 100
    G = 300 @ 8msg/op

    mops ∝ 2 * dim_topology * (in_msg + out_msg);
    let gossip be the message;
    mops ∝ 2 * 2 * (1 + 1) -> 8 msgs per gossip.

    let mops max: 20
    20 = 8 * G_count -> 20/8 = 2.5

    est. min gossip; max_gossip * 8 / 20 = 120

*/
// range [120, 300]
pub const GOSSIP_INTERVAL: u64 = 120;
pub const EXPECT_ELEMENTS: usize = 1_100;
/*
    obj. reduce max latency.
    let elements; e : 1_000
    probability of success per hop (1 - fp)
    (1-fp)^(sqrt(n))

    let ε : prob of one element not missing after grid n hops.
    (1-fp)^ 5 = ε

    corners have 2 incoming edges, prob to succeed at least one.
        $$P(X = k) = \binom{n}{k} p^k (1-p)^{n-k}$$

    P(x): 1 - (1 - ε)**2

    suceed for all numbers P(x)**e
    P(x | e)**1_000

   Satisfy no misses at the grid corners.

    P(x | 0.01)**1_000 == 0.09027772028293109
    P(x | 0.005)**1_000 == 0.5418251381604163
    P(x | 0.001)**1_000 == 0.975406950667474
*/
pub const FP_RATE: f64 = 0.001;

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
        messages: BTreeSet<u64>,
        in_reply_to: u64,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk {
        in_reply_to: u64,
    },
    Gossip {
        #[serde(with = "bloom_serde")]
        bloom: Bloom<u64>,
    },
    GossipOk {
        messages: Vec<u64>,
    },
}

impl Reply<Payload, &mut Node> for Message<Payload> {
    fn reply(self, state: &mut Node) -> Option<Payload> {
        match self.body.payload {
            Payload::Broadcast { message } => {
                state.messages.insert(message);
                state.index.set(&message);
                Some(Payload::BroadcastOk {
                    in_reply_to: self.body.msg_id,
                })
            }
            Payload::BroadcastOk { .. } => unreachable!(),
            Payload::Read => Some(Payload::ReadOk {
                messages: state.messages.clone(),
                in_reply_to: self.body.msg_id,
            }),
            Payload::ReadOk { .. } => unreachable!(),
            Payload::Topology { mut topology } => {
                state.neighborhood = topology
                    .remove(&state.node_id)
                    .unwrap_or_else(|| panic!("topology for {:?}", state.node_id));
                Some(Payload::TopologyOk {
                    in_reply_to: self.body.msg_id,
                })
            }
            Payload::TopologyOk { .. } => unreachable!(),
            Payload::Gossip { bloom } => {
                let dest_diff: Vec<u64> = state
                    .messages
                    .iter()
                    .filter(|m| !bloom.check(m))
                    .copied()
                    .collect();

                Some(Payload::GossipOk {
                    messages: dest_diff,
                })
            }
            Payload::GossipOk { messages } => {
                for m in &messages {
                    state.index.set(m);
                }
                state.messages.extend(messages);
                None
            }
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
                messages: BTreeSet::from([1, 2, 3]),
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
        topology.insert("n0".to_string(), vec![format!("n1")]);
        let obj = Body {
            msg_id: 12,
            payload: Payload::Topology { topology },
        };
        assert_eq!(
            serde_json::to_string(&obj).unwrap(),
            r#"{"msg_id":12,"type":"topology","topology":{"n0":["n1"]}}"#
        );
    }

    #[test]
    fn gossip_ok_format() {
        let messages = Vec::from([1, 2, 3]);
        let body = Body {
            msg_id: 12,
            payload: Payload::GossipOk { messages },
        };

        assert_eq!(
            serde_json::to_string(&body).unwrap(),
            r#"{"msg_id":12,"type":"gossip_ok","messages":[1,2,3]}"#
        );
    }
}
