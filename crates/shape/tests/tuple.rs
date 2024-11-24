use shape::{Shape, ShapeOptions, Tuple, Type};

mod common;

#[test]
fn tuple() {
  let expected = Type::Tuple(Tuple {
    items: vec![Type::Number, Type::String],
    rest: None,
  });

  eq!(<(i32, String)>::shape(&ShapeOptions::for_serialize()), expected);
  eq!(<(i32, String)>::shape(&ShapeOptions::for_deserialize()), expected);
}