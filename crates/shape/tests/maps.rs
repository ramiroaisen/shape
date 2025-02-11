mod common;

use std::collections::{BTreeMap, HashMap};

#[cfg(feature = "indexmap")]
use indexmap::IndexMap;

use shape::{Record, Shape, ShapeOptions, Type};

#[test]
fn maps() {
  eq!(HashMap::<String, i32>::shape(&ShapeOptions::for_serialize()), Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::String),
    value: Box::new(Type::Number),
  }));

  #[cfg(feature = "indexmap")]
  eq!(IndexMap::<String, i32>::shape(&ShapeOptions::for_serialize()), Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::String),
    value: Box::new(Type::Number),
  }));

  eq!(BTreeMap::<String, i32>::shape(&ShapeOptions::for_serialize()), Type::Record(
    Record {
      optional: false,
      readonly: false,
      key: Box::new(Type::String),
      value: Box::new(Type::Number),
  }));
}