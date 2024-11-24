mod common;

use indexmap::IndexMap;
use shape::{Object, Property, Shape, ShapeOptions, Type};

#[test]
fn generics() {

  #[derive(Shape)]
  #[allow(unused)]
  struct Generic<'a, T> {
    field: &'a T,
  } 
  
  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "field".into(),
      Property {
        ty: Type::String,
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(Generic::<String>::shape(&ShapeOptions::for_serialize()), expected);
}

#[test]
fn generics_with_where_clause() {
  #[derive(Shape)]
  #[allow(unused)]
  struct Generic<'a, T> where T: Default {
    field: &'a T,
  } 
  
  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "field".into(),
      Property {
        ty: Type::String,
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(Generic::<String>::shape(&ShapeOptions::for_serialize()), expected);
}