mod common;

use std::collections::{BTreeMap, HashMap};

use indexmap::IndexMap;
use shape::{Record, Shape, ShapeOptions, Type};

#[test]
fn maps() {
  eq!(HashMap::<String, i32>::shape(&ShapeOptions::for_serialize()), Type::Record(Record {
    key: Box::new(Type::String),
    value: Box::new(Type::Number),
  }));

  eq!(IndexMap::<String, i32>::shape(&ShapeOptions::for_serialize()), Type::Record(Record {
    key: Box::new(Type::String),
    value: Box::new(Type::Number),
  }));

  eq!(BTreeMap::<String, i32>::shape(&ShapeOptions::for_serialize()), Type::Record(Record {
    key: Box::new(Type::String),
    value: Box::new(Type::Number),
  }));
}