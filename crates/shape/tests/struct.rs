use indexmap::IndexMap;
use shape::{Object, Shape, ShapeOptions, Type};

mod common;

#[test]
fn empty_struct() {
  #[derive(Shape)]
  #[allow(unused)]
  struct EmptyStruct {}

  let expected = Type::Object(Object {
    properties: IndexMap::new(),
  });

  eq!(EmptyStruct::shape(&ShapeOptions::for_serialize()), expected);
  eq!(EmptyStruct::shape(&ShapeOptions::for_deserialize()), expected);
}