mod common;

use std::collections::{BTreeSet, HashSet};

use indexmap::IndexSet;
use shape::{Array, Shape, ShapeOptions, Tuple, Type};

#[test]
fn array_containers() {
  
  let expected = Type::Array(Array {
    item: Box::new(Type::String),
  });
  
  eq!(Vec::<String>::shape(&ShapeOptions::Serialize), expected);
  eq!(Vec::<String>::shape(&ShapeOptions::Deserialize), expected);

  eq!(<[String]>::shape(&ShapeOptions::Serialize), expected);
  eq!(<[String]>::shape(&ShapeOptions::Deserialize), expected);

  eq!(HashSet::<String>::shape(&ShapeOptions::Serialize), expected);
  eq!(HashSet::<String>::shape(&ShapeOptions::Deserialize), expected);

  eq!(<BTreeSet<String>>::shape(&ShapeOptions::Serialize), expected);
  eq!(<BTreeSet<String>>::shape(&ShapeOptions::Deserialize), expected);

  eq!(<IndexSet<String>>::shape(&ShapeOptions::Serialize), expected);
  eq!(<IndexSet<String>>::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn static_array() {
  let expected = Type::Tuple(
    Tuple {
      items: vec![Type::Number, Type::Number, Type::Number],
      rest: None,
    }
  );

  eq!(<[i32; 3]>::shape(&ShapeOptions::Serialize), expected);
  eq!(<[i32; 3]>::shape(&ShapeOptions::Deserialize), expected);
}