use std::{error::Error, fmt, marker::PhantomData};

use super::*;

use serde::{
    de::{Error as DeError, Visitor},
    Deserializer, Serializer,
};

pub fn serialize<S: Serializer, T: ?Sized>(
    bloom: &Bloom<T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let bytes = bloom.as_slice();
    let x = hex::encode(bytes);
    serializer.serialize_str(&x)
}

struct BloomVisitor<T: ?Sized> {
    _phantom: PhantomData<T>,
}

impl<'de, T: ?Sized> Visitor<'de> for BloomVisitor<T> {
    type Value = Bloom<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Blom filter")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let x = hex::decode(v).unwrap();
        Bloom::from_slice(&x).map_err(|e| {
            DeError::custom(e)
        })
    }
}

pub fn deserialize<'de, D: Deserializer<'de>, T: ?Sized>(
    deserializer: D,
) -> Result<Bloom<T>, D::Error> {
    deserializer.deserialize_bytes(BloomVisitor {
        _phantom: PhantomData,
    })
}

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use bloomfilter::Bloom;

    use crate::message::broadcast::Payload;

    #[test]
    fn serializing_to_string() {
        let filter = Bloom::new_for_fp_rate(10_000, 0.5).unwrap();
        let msg = Payload::Gossip {
            bloom: filter.clone(),
        };
        let x = serde_json::to_string(&msg).unwrap();
        println!("{:?}", x);
        let back: Payload = serde_json::from_str(&x).unwrap();
        assert_matches!(back,Payload::Gossip { bloom } => {
            assert_eq!(bloom.as_slice(), filter.as_slice());
        })
    }
}
