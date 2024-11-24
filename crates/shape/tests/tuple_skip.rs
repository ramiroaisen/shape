use serde::Serialize;
use serde_json::json;
use shape::{Shape, ShapeOptions, Tuple, Type};

mod common;

#[allow(unused)]
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
  value == &T::default()
}

#[test]
fn serde_tuple_skip() {
  #[derive(Serialize, Shape)]
  struct Ty(String, #[serde(skip_serializing_if="is_default")] u8, bool);

  assert_eq!(
    serde_json::to_value(Ty(String::from("a"), 0, true)).unwrap(),
    json!(["a", true])
  );

  assert_eq!(
    serde_json::to_value(Ty(String::from("a"), 1, true)).unwrap(),
    json!(["a", 1, true])
  );
}

#[test]
fn tuple_skip_if() {
  #[derive(Serialize, Shape)]
  struct Ty(String, #[serde(skip_serializing_if="is_default")] u8, bool);

  eq!(
    Ty::shape(&ShapeOptions::for_serialize()),
    Type::Or(vec![
      Type::Tuple(
        Tuple {
          items: vec![
            Type::String,
            Type::Boolean,
          ],
          rest: None,
        }
      ),
      Type::Tuple(
        Tuple {
          items: vec![
            Type::String,
            Type::Number,
            Type::Boolean,
          ],
          rest: None,
        }
      )
    ])
  );
}

#[test]
fn tuple_skip_if_two() {
  #[derive(Shape)]
  #[allow(unused)]
  struct Ty(String, #[serde(skip_serializing_if="is_default")] u8, #[serde(skip_serializing_if="is_default")] bool);

  eq!(
    Ty::shape(&ShapeOptions::for_serialize()),
    Type::Or(vec![
      Type::Tuple(
        Tuple {
          items: vec![ Type::String ],
          rest: None,
        }
      ),
      Type::Tuple(
        Tuple {
          items: vec![
            Type::String,
            Type::Number,
          ],
          rest: None,
        }
      ),
      Type::Tuple(
        Tuple {
          items: vec![
            Type::String,
            Type::Boolean,
          ],
          rest: None,
        }
      ),
      Type::Tuple(
        Tuple {
          items: vec![
            Type::String,
            Type::Number,
            Type::Boolean,
          ],
          rest: None,
        }
      )
    ])
  );
}

#[test]
fn tuple_skip_deserializing() {
  #[derive(Serialize, Shape)]
  struct Ty(String, #[serde(skip_deserializing)] u8, bool);

  eq!(
    Ty::shape(&ShapeOptions::for_deserialize()),
    Type::Tuple(
      Tuple {
        items: vec![
          Type::String,
          Type::Boolean,
        ],
        rest: None,
      }
    )
  );
}

#[test]
fn tuple_default() {
  #[derive(Serialize, Shape)]
  struct Ty(String, u8, #[serde(default)] bool);

  eq!(
    Ty::shape(&ShapeOptions::for_deserialize()),
    Type::Or(vec![
      Type::Tuple(
        Tuple {
          items: vec![
            Type::String,
            Type::Number,
          ],
          rest: None,
        }
      ),
      Type::Tuple(
        Tuple {
          items: vec![
            Type::String,
            Type::Number,
            Type::Boolean,
          ],
          rest: None,
        }
      )
    ])
  );
}


#[test]
fn tuple_default_two() {
  #[derive(Serialize, Shape)]
  struct Ty(String, #[serde(default)] u8, #[serde(default)] bool);

  eq!(
    Ty::shape(&ShapeOptions::for_deserialize()),
    Type::Or(vec![
      Type::Tuple(
        Tuple {
          items: vec![ Type::String ],
          rest: None,
        }
      ),
      Type::Tuple(
        Tuple {
          items: vec![
            Type::String,
            Type::Number,
          ],
          rest: None,
        }
      ),
      Type::Tuple(
        Tuple {
          items: vec![
            Type::String,
            Type::Number,
            Type::Boolean,
          ],
          rest: None,
        }
      )
    ])
  );
}