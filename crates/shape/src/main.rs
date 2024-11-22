use shape::{Shape, ShapeOptions};

#[derive(Shape)]
pub struct Struct<'a> {
  #[shape(rename = "b")]
  pub a: i32,

  pub s: &'a String,
}

fn main() {
  let options = ShapeOptions::Serialize;
  Struct::shape(&options);
}