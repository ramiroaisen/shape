use indexmap::IndexMap;
use shape::{Object, Property, ShapeOptions, Type};
use shape::Shape;

mod common;

// snake_case
#[test]
fn snake_case() {
  
  #[derive(Shape)]
  #[allow(non_snake_case)]
  #[allow(unused)]
  #[serde(rename_all = "snake_case")]
  struct Snake {
    pub SomeField: u8
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "some_field".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ]),
  });

  eq!(Snake::shape(&shape::ShapeOptions::for_serialize()), expected);
  eq!(Snake::shape(&shape::ShapeOptions::for_deserialize()), expected);
}

// kebab-case
#[test]
fn kebab_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all = "kebab-case")]
  struct Kebab {
    pub some_field: u8
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "some-field".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ])
  });

  eq!(Kebab::shape(&shape::ShapeOptions::for_serialize()), expected);
  eq!(Kebab::shape(&shape::ShapeOptions::for_deserialize()), expected);
}

// SCREAMING_SNAKE_CASE
#[test]
fn screaming_snake_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[allow(non_snake_case)]
  #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
  struct ScreamingSnake {
    pub SomeField: u8
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "SOME_FIELD".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ])
  });

  eq!(ScreamingSnake::shape(&shape::ShapeOptions::for_serialize()), expected);
  eq!(ScreamingSnake::shape(&shape::ShapeOptions::for_deserialize()), expected);
}

// PascalCase
#[test]
fn pascal_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[allow(non_snake_case)]
  #[serde(rename_all = "PascalCase")]
  struct Pascal {
    pub some_field: u8
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "SomeField".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ])
  });

  eq!(Pascal::shape(&shape::ShapeOptions::for_serialize()), expected);
  eq!(Pascal::shape(&shape::ShapeOptions::for_deserialize()), expected);
}

// camelCase
#[test]
fn camel_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[allow(non_snake_case)]
  #[serde(rename_all = "camelCase")]
  struct Camel {
    pub some_field: u8
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "someField".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ])
  });

  eq!(Camel::shape(&shape::ShapeOptions::for_serialize()), expected);
  eq!(Camel::shape(&shape::ShapeOptions::for_deserialize()), expected);
}

// SCHREAMING-KEBAB-CASE
#[test]
fn screaming_kebab_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[allow(non_snake_case)]
  #[serde(rename_all = "SCREAMING-KEBAB-CASE")]
  struct ScreamingKebab {
    pub some_field: u8
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "SOME-FIELD".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ])
  });

  eq!(ScreamingKebab::shape(&shape::ShapeOptions::for_serialize()), expected);
  eq!(ScreamingKebab::shape(&shape::ShapeOptions::for_deserialize()), expected);
}

// UPPERCASE
#[test]
fn uppercase() {
  #[derive(Shape)]
  #[allow(unused)]
  #[allow(non_snake_case)]
  #[serde(rename_all = "UPPERCASE")]
  struct Uppercase {
    pub SomeField: u8
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "SOMEFIELD".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ])
  });

  eq!(Uppercase::shape(&shape::ShapeOptions::for_serialize()), expected);
  eq!(Uppercase::shape(&shape::ShapeOptions::for_deserialize()), expected);
}

// lowercase
#[test]
fn lowercase() {
  #[derive(Shape)]
  #[allow(unused)]
  #[allow(non_snake_case)]
  #[serde(rename_all = "lowercase")]
  struct Lowercase {
    pub some_field: u8
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "some_field".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ]),
  });

  eq!(Lowercase::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Lowercase::shape(&ShapeOptions::for_deserialize()), expected);
}