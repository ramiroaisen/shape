use shape::{Shape, ShapeOptions, Tuple, Type};

mod common;

#[test]
fn tuple_primitive() {
  #[derive(Shape)]
  #[allow(unused)]
  struct TupleStruct(String, i32);

  let expected = Type::Tuple(Tuple {
    items: vec![Type::String, Type::Number],
    rest: None,
  });

  eq!(TupleStruct::shape(&ShapeOptions::Serialize), expected);
  eq!(TupleStruct::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn tuple_skip() {
  #[derive(Shape)]
  #[allow(unused)]
  struct TupleStruct(String, #[serde(skip)] i32);

  let expected = Type::Tuple(Tuple {
    items: vec![Type::String],
    rest: None,
  });

  eq!(TupleStruct::shape(&ShapeOptions::Serialize), expected);
  eq!(TupleStruct::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn tuple_skip_serializing() {
  #[derive(Shape)]
  #[allow(unused)]
  struct TupleStruct(String, #[serde(skip_serializing)] i32);

  let ser = Type::Tuple(Tuple {
    items: vec![Type::String],
    rest: None,
  });

  let de = Type::Tuple(Tuple {
    items: vec![Type::String, Type::Number],
    rest: None,
  });

  eq!(TupleStruct::shape(&ShapeOptions::Serialize), ser);
  eq!(TupleStruct::shape(&ShapeOptions::Deserialize), de);
}

#[test]
fn tuple_skip_deserializing() {
  #[derive(Shape)]
  #[allow(unused)]
  struct TupleStruct(String, #[serde(skip_deserializing)] i32);

  let de = Type::Tuple(Tuple {
    items: vec![Type::String],
    rest: None,
  });

  let ser = Type::Tuple(Tuple {
    items: vec![Type::String, Type::Number],
    rest: None,
  });

  eq!(TupleStruct::shape(&ShapeOptions::Serialize), ser);
  eq!(TupleStruct::shape(&ShapeOptions::Deserialize), de);
}

#[test]
fn tuple_empty_skip_one() {
  #[derive(Shape)]
  #[allow(unused)]
  struct TupleStruct(#[serde(skip)] i32);

  let expected = Type::Tuple(Tuple {
    items: vec![],
    rest: None,
  });

  eq!(TupleStruct::shape(&ShapeOptions::Serialize), expected);
  eq!(TupleStruct::shape(&ShapeOptions::Deserialize), expected);
}