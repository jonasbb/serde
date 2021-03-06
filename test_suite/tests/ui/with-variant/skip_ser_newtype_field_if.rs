use serde_derive::Serialize;

#[derive(Serialize)]
enum Enum {
    #[serde(serialize_with = "serialize_some_newtype_variant")]
    Newtype(#[serde(skip_serializing_if = "always")] String),
}

fn main() {}
