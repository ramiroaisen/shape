mod common;

use std::{rc::Rc, sync::Arc};
use shape::{Shape, ShapeOptions, Type};

#[test]
fn containers() {
  eq!(Box::<String>::shape(&ShapeOptions::for_serialize()), Type::String);
  eq!(Arc::<u8>::shape(&ShapeOptions::for_serialize()), Type::Number);
  eq!(Rc::<bool>::shape(&ShapeOptions::for_serialize()), Type::Boolean);
}