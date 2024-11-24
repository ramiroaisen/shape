mod common;

use std::{rc::Rc, sync::Arc};
use shape::{Shape, ShapeOptions, Type};

#[test]
fn containers() {
  eq!(Box::<String>::shape(&ShapeOptions::Serialize), Type::String);
  eq!(Arc::<u8>::shape(&ShapeOptions::Serialize), Type::Number);
  eq!(Rc::<bool>::shape(&ShapeOptions::Serialize), Type::Boolean);
}