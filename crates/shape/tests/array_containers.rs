mod common;

use std::collections::{BTreeSet, HashSet};

#[cfg(feature = "indexmap")]
use indexmap::IndexSet;

use shape::{Array, Shape, ShapeOptions, Tuple, Type};

#[test]
fn array_containers() {
  
  let expected = Type::Array(Array {
    item: Box::new(Type::String),
  });
  
  eq!(Vec::<String>::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Vec::<String>::shape(&ShapeOptions::for_deserialize()), expected);

  eq!(<[String]>::shape(&ShapeOptions::for_serialize()), expected);
  eq!(<[String]>::shape(&ShapeOptions::for_deserialize()), expected);

  eq!(HashSet::<String>::shape(&ShapeOptions::for_serialize()), expected);
  eq!(HashSet::<String>::shape(&ShapeOptions::for_deserialize()), expected);

  eq!(<BTreeSet<String>>::shape(&ShapeOptions::for_serialize()), expected);
  eq!(<BTreeSet<String>>::shape(&ShapeOptions::for_deserialize()), expected);

  #[cfg(feature = "indexmap")]
  eq!(<IndexSet<String>>::shape(&ShapeOptions::for_serialize()), expected);
  #[cfg(feature = "indexmap")]
  eq!(<IndexSet<String>>::shape(&ShapeOptions::for_deserialize()), expected);
}

#[test]
fn static_array() {
  let expected = Type::Tuple(
    Tuple {
      items: vec![Type::Number, Type::Number, Type::Number],
      rest: None,
    }
  );

  eq!(<[i32; 3]>::shape(&ShapeOptions::for_serialize()), expected);
  eq!(<[i32; 3]>::shape(&ShapeOptions::for_deserialize()), expected);
}