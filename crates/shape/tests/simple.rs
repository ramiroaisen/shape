mod common;
use shape::{Shape, ShapeOptions, Type};

#[test]
fn primitives() {
  for options in [ShapeOptions::for_serialize(), ShapeOptions::for_deserialize()] {
    eq!(Type::String, String::shape(&options));
    eq!(Type::String, str::shape(&options));
    eq!(Type::Number, u8::shape(&options));
    eq!(Type::Number, u16::shape(&options));
    eq!(Type::Number, u32::shape(&options));
    eq!(Type::Number, u64::shape(&options));
    eq!(Type::Number, u128::shape(&options));
    eq!(Type::Number, usize::shape(&options));
    eq!(Type::Number, i8::shape(&options));
    eq!(Type::Number, i16::shape(&options));
    eq!(Type::Number, i32::shape(&options));
    eq!(Type::Number, i64::shape(&options));
    eq!(Type::Number, i128::shape(&options));
    eq!(Type::Number, isize::shape(&options));
    eq!(Type::Number, f32::shape(&options));
    eq!(Type::Number, f64::shape(&options));
    eq!(Type::Boolean, bool::shape(&options));
    eq!(Type::Null, <()>::shape(&options));
  }
}

#[test]
fn refs() {
  for options in [ShapeOptions::for_serialize(), ShapeOptions::for_deserialize()] {
    eq!(Type::String, <&String>::shape(&options));
    eq!(Type::String, <&str>::shape(&options));
    eq!(Type::Number, <&u8>::shape(&options));
    eq!(Type::Number, <&u16>::shape(&options));
    eq!(Type::Number, <&u32>::shape(&options));
    eq!(Type::Number, <&u64>::shape(&options));
    eq!(Type::Number, <&u128>::shape(&options));
    eq!(Type::Number, <&usize>::shape(&options));
    eq!(Type::Number, <&i8>::shape(&options));
    eq!(Type::Number, <&i16>::shape(&options));
    eq!(Type::Number, <&i32>::shape(&options));
    eq!(Type::Number, <&i64>::shape(&options));
    eq!(Type::Number, <&i128>::shape(&options));
    eq!(Type::Number, <&isize>::shape(&options));
    eq!(Type::Number, <&f32>::shape(&options));
    eq!(Type::Number, <&f64>::shape(&options));
    eq!(Type::Boolean, <&bool>::shape(&options));
    eq!(Type::Null, <&()>::shape(&options));
  }
}

#[test]
fn unit_struct() {
  #[derive(Shape)]
  struct UnitStruct;

  eq!(UnitStruct::shape(&ShapeOptions::for_serialize()), Type::Null);
  eq!(UnitStruct::shape(&ShapeOptions::for_deserialize()), Type::Null);
}

#[test]
fn unit() {
  eq!(<()>::shape(&ShapeOptions::for_serialize()), Type::Null);
  eq!(<()>::shape(&ShapeOptions::for_deserialize()), Type::Null);
}