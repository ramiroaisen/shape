mod common;
use std::vec;

use shape::{Shape, ShapeOptions, ShapeOptionsKind, Type};

#[test]
fn options_none() {
  let ser_opts = ShapeOptions {
    kind: ShapeOptionsKind::Serialize,
    option_is_optional: false,
    option_add_undefined: false,
    option_add_null: false,
  };

  let de_opts = ShapeOptions {
    kind: ShapeOptionsKind::Deserialize,
    option_is_optional: false,
    option_add_undefined: false,
    option_add_null: false,
  };

  eq!(Option::<String>::shape(&ser_opts), Type::String);
  eq!(Option::<String>::shape(&de_opts), Type::String);
}

#[test]
fn options_is_optional() {
  
  #[allow(unused)]
  #[derive(Shape)]
  struct Struct {
    field: Option<String>,
  }

  let ser_opts = ShapeOptions {
    kind: ShapeOptionsKind::Serialize,
    option_is_optional: true,
    option_add_undefined: false,
    option_add_null: false,
  };

  let de_opts = ShapeOptions {
    kind: ShapeOptionsKind::Deserialize,
    option_is_optional: true,
    option_add_undefined: false,
    option_add_null: false,
  };

  let expected = Type::Object(shape::Object {
    properties: shape::indexmap::IndexMap::from([(
      "field".into(),
      shape::Property {
        ty: Type::String,
        optional: true,
        readonly: false,
      },
    )]),
  });

  eq!(Struct::shape(&ser_opts), expected);
  eq!(Struct::shape(&de_opts), expected);
}


#[test]
fn options_add_undefined() {
  
  #[allow(unused)]
  #[derive(Shape)]
  struct Struct {
    field: Option<String>,
  }

  let ser_opts = ShapeOptions {
    kind: ShapeOptionsKind::Serialize,
    option_is_optional: false,
    option_add_undefined: true,
    option_add_null: false,
  };

  let de_opts = ShapeOptions {
    kind: ShapeOptionsKind::Deserialize,
    option_is_optional: false,
    option_add_undefined: true,
    option_add_null: false,
  };

  let expected = Type::Object(shape::Object {
    properties: shape::indexmap::IndexMap::from([(
      "field".into(),
      shape::Property {
        ty: Type::Or(vec![Type::String, Type::Undefined]),
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(Struct::shape(&ser_opts), expected);
  eq!(Struct::shape(&de_opts), expected);
}

#[test]
fn options_add_null() {
  
  #[allow(unused)]
  #[derive(Shape)]
  struct Struct {
    field: Option<String>,
  }

  let ser_opts = ShapeOptions {
    kind: ShapeOptionsKind::Serialize,
    option_is_optional: false,
    option_add_undefined: false,
    option_add_null: true,
  };

  let de_opts = ShapeOptions {
    kind: ShapeOptionsKind::Deserialize,
    option_is_optional: false,
    option_add_undefined: false,
    option_add_null: true,
  };

  let expected = Type::Object(shape::Object {
    properties: shape::indexmap::IndexMap::from([(
      "field".into(),
      shape::Property {
        ty: Type::Or(vec![Type::String, Type::Null]),
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(Struct::shape(&ser_opts), expected);
  eq!(Struct::shape(&de_opts), expected);
}

#[test]
fn options_add_null_and_undefined() {
  
  #[allow(unused)]
  #[derive(Shape)]
  struct Struct {
    field: Option<String>,
  }

  let ser_opts = ShapeOptions {
    kind: ShapeOptionsKind::Serialize,
    option_is_optional: false,
    option_add_undefined: true,
    option_add_null: true,
  };

  let de_opts = ShapeOptions {
    kind: ShapeOptionsKind::Deserialize,
    option_is_optional: false,
    option_add_undefined: true,
    option_add_null: true,
  };

  let expected = Type::Object(shape::Object {
    properties: shape::indexmap::IndexMap::from([(
      "field".into(),
      shape::Property {
        ty: Type::Or(vec![Type::String, Type::Null, Type::Undefined]),
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(Struct::shape(&ser_opts), expected);
  eq!(Struct::shape(&de_opts), expected);
}