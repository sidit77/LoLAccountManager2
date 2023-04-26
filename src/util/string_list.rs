use std::borrow::Cow;
use std::fmt::Formatter;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserializer, Serializer};

pub fn serialize<S: Serializer>(data: &str, s: S) -> Result<S::Ok, S::Error> {
    let mut seq = s.serialize_seq(None)?;
    for line in data.lines() {
        seq.serialize_element(line)?;
    }
    seq.end()
}

#[derive(Debug)]
struct StringReader;

impl<'de> Visitor<'de> for StringReader {
    type Value = String;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a list of strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'de> {
        let mut buffer = String::new();
        while let Some(x) = seq.next_element::<Cow<'de, str>>()? {
            buffer.push_str(&x);
            buffer.push('\n');
        }
        Ok(buffer)
    }

}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    d.deserialize_seq(StringReader)
}