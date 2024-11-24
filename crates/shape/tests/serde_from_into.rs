use shape::{Shape, ShapeOptions, Type};

mod common;

#[test]
fn serde_from_into() {
  #[derive(Shape)]
  #[serde(into = "String")]
  #[serde(from = "u8")]
  #[allow(unused)]
  struct SerdeFromInto {
    #[serde(rename = "some_field")]
    some_field: u8,
  }

  eq!(SerdeFromInto::shape(&ShapeOptions::for_serialize()), Type::String);
  eq!(SerdeFromInto::shape(&ShapeOptions::for_deserialize()), Type::Number);
}

#[test]
fn serde_try_from_try_into() {
  #[derive(Shape)]
  #[serde(try_from = "String")]
  #[serde(try_into = "u8")]
  #[allow(unused)]
  struct SerdeTryFromTryInto {
    #[serde(rename = "some_field")]
    some_field: u8,
  }

  eq!(SerdeTryFromTryInto::shape(&ShapeOptions::for_serialize()), Type::Number);
  eq!(SerdeTryFromTryInto::shape(&ShapeOptions::for_deserialize()), Type::String);
}