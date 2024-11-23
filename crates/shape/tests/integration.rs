use indexmap::IndexMap;
use shape::{Array, Literal, Object, Property, Shape, ShapeOptions, Tuple, Type};
use text_diff::print_diff;

macro_rules! eq {
  ($a:expr, $b:expr) => {
    let a = &$a;
    let b = &$b;

    let debug_a = format!("{:?}", &a);
    let debug_b = format!("{:?}", &b);

    if a != b {
      print_diff(&debug_a, &debug_b, " ");
      panic!("$a != $b");
    }
  };
}

#[test]
fn primitives() {
  for options in [ShapeOptions::Serialize, ShapeOptions::Deserialize] {
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
  for options in [ShapeOptions::Serialize, ShapeOptions::Deserialize] {
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
fn simple_struct() {
  #[allow(unused)]
  #[derive(Shape)]
  struct Simple<'a> {
    bool: bool,
    u8: u8,
    str: &'a str,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "bool".into(),
        Property {
          ty: Type::Boolean,
          optional: false,
          readonly: false,
        },
      ),
      (
        "u8".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
      (
        "str".into(),
        Property {
          ty: Type::String,
          optional: false,
          readonly: false,
        },
      ),
    ]),
  });

  eq!(Simple::shape(&ShapeOptions::Serialize), expected);
  eq!(Simple::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn struct_with_serde_attrs() {
  #[derive(Shape)]
  #[serde(rename_all = "camelCase")]
  #[allow(unused)]
  struct SerdeTest {
    first_field: String,

    #[serde(skip)]
    skipped: u32,

    #[serde(rename = "opt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "asd")]
    optional_value: bool,

    nested_struct: NestedStruct,
  }

  #[derive(Shape)]
  #[serde(rename_all = "kebab-case")]
  #[allow(unused)]
  struct NestedStruct {
    field_one: String,
    field_two: i32,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "firstField".into(),
        Property {
          ty: Type::String,
          optional: false,
          readonly: false,
        },
      ),
      (
        "opt".into(),
        Property {
          ty: Type::Boolean,
          optional: true,
          readonly: false,
        },
      ),
      (
        "nestedStruct".into(),
        Property {
          ty: Type::Object(Object {
            properties: IndexMap::from([
              (
                "field-one".into(),
                Property {
                  ty: Type::String,
                  optional: false,
                  readonly: false,
                },
              ),
              (
                "field-two".into(),
                Property {
                  ty: Type::Number,
                  optional: false,
                  readonly: false,
                },
              ),
            ]),
          }),
          optional: false,
          readonly: false,
        },
      ),
    ]),
  });

  eq!(SerdeTest::shape(&ShapeOptions::Serialize), expected);
  eq!(SerdeTest::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn struct_with_flatten() {
  #[derive(Shape)]
  #[allow(unused)]
  struct Wrapper {
    top_level: String,
    #[serde(flatten)]
    flattened: Flattened,
  }

  #[derive(Shape)]
  #[allow(unused)]
  struct Flattened {
    nested_field: i32,
    another_field: bool,
  }

  let expected = Type::And(vec![
    Type::Object(Object {
      properties: IndexMap::from([(
        "top_level".into(),
        Property {
          ty: Type::String,
          optional: false,
          readonly: false,
        },
      )]),
    }),
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "nested_field".into(),
          Property {
            ty: Type::Number,
            optional: false,
            readonly: false,
          },
        ),
        (
          "another_field".into(),
          Property {
            ty: Type::Boolean,
            optional: false,
            readonly: false,
          },
        ),
      ]),
    }),
  ]);

  eq!(Wrapper::shape(&ShapeOptions::Serialize), expected);
  eq!(Wrapper::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn newtype() {
  #[derive(Shape)]
  struct Number(pub i32);

  let expected = Type::Number;
  eq!(Number::shape(&ShapeOptions::Serialize), expected);
  eq!(Number::shape(&ShapeOptions::Deserialize), expected);

  #[derive(Shape)]
  struct Str<'a>(&'a str);

  let expected = Type::String;
  eq!(String::shape(&ShapeOptions::Serialize), expected);
  eq!(String::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn enum_untagged() {
  #[derive(Shape)]
  #[serde(untagged)]
  enum Enum {
    A,
    B(i32),
    C(u8, String),
    D { a: i32, b: String },
  }

  let expected = Type::Or(vec![
    Type::Null,
    Type::Number,
    Type::Tuple(Tuple {
      items: vec![Type::Number, Type::String],
      rest: None,
    }),
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "a".into(),
          Property {
            ty: Type::Number,
            optional: false,
            readonly: false,
          },
        ),
        (
          "b".into(),
          Property {
            ty: Type::String,
            optional: false,
            readonly: false,
          },
        ),
      ]),
    }),
  ]);

  eq!(Enum::shape(&ShapeOptions::Serialize), expected);
  eq!(Enum::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn enum_internally_tagged() {
  #[derive(Shape)]
  #[serde(tag = "type")]
  enum Enum {
    A,
    B(i32),
    C(u8, bool),
    D { a: i32, b: String },
  }

  let expected = Type::Or(vec![
    // unit
    Type::Object(Object {
      properties: IndexMap::from([(
        "type".into(),
        Property {
          ty: Type::Literal(Literal::String("A".into())),
          optional: false,
          readonly: false,
        },
      )]),
    }),
    // newtype
    Type::And(vec![
      Type::Object(Object {
        properties: IndexMap::from([(
          "type".into(),
          Property {
            ty: Type::Literal(Literal::String("B".into())),
            optional: false,
            readonly: false,
          },
        )]),
      }),
      Type::Number,
    ]),
    // tuple
    Type::And(vec![
      Type::Object(Object {
        properties: IndexMap::from([(
          "type".into(),
          Property {
            ty: Type::Literal(Literal::String("C".into())),
            optional: false,
            readonly: false,
          },
        )]),
      }),
      Type::Tuple(Tuple {
        items: vec![Type::Number, Type::Boolean],
        rest: None,
      }),
    ]),
    // named
    Type::And(vec![
      Type::Object(Object {
        properties: IndexMap::from([(
          "type".into(),
          Property {
            ty: Type::Literal(Literal::String("D".into())),
            optional: false,
            readonly: false,
          },
        )]),
      }),
      Type::Object(Object {
        properties: IndexMap::from([
          (
            "a".into(),
            Property {
              ty: Type::Number,
              optional: false,
              readonly: false,
            },
          ),
          (
            "b".into(),
            Property {
              ty: Type::String,
              optional: false,
              readonly: false,
            },
          ),
        ]),
      }),
    ]),
  ]);

  eq!(Enum::shape(&ShapeOptions::Serialize), expected);
  eq!(Enum::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn enum_adjacently_tagged() {
  #[derive(Shape)]
  #[serde(tag = "type", content = "content")]
  enum Enum {
    A,
    B(i32),
    C(u8, bool),
    D { a: i32, b: String },
  }

  let expected = Type::Or(vec![
    // unit
    Type::Object(Object {
      properties: IndexMap::from([(
        "type".into(),
        Property {
          ty: Type::Literal(Literal::String("A".into())),
          optional: false,
          readonly: false,
        },
      )]),
    }),
    // newtype
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "type".into(),
          Property {
            ty: Type::Literal(Literal::String("B".into())),
            optional: false,
            readonly: false,
          },
        ),
        (
          "content".into(),
          Property {
            ty: Type::Number,
            optional: false,
            readonly: false,
          },
        ),
      ]),
    }),
    // tuple
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "type".into(),
          Property {
            ty: Type::Literal(Literal::String("C".into())),
            optional: false,
            readonly: false,
          },
        ),
        (
          "content".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Tuple(Tuple {
              items: vec![Type::Number, Type::Boolean],
              rest: None,
            }),
          },
        ),
      ]),
    }),
    // named
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "type".into(),
          Property {
            ty: Type::Literal(Literal::String("D".into())),
            optional: false,
            readonly: false,
          },
        ),
        (
          "content".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "a".into(),
                  Property {
                    ty: Type::Number,
                    optional: false,
                    readonly: false,
                  },
                ),
                (
                  "b".into(),
                  Property {
                    ty: Type::String,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            }),
          },
        ),
      ]),
    }),
  ]);

  eq!(Enum::shape(&ShapeOptions::Serialize), expected);
  eq!(Enum::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn struct_with_rename() {
  #[derive(Shape)]
  #[serde(rename_all = "snake_case")]
  #[allow(unused)]
  struct RenamedFields {
    originalField: String,
    anotherField: i32,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([
      (
        "original_field".into(),
        Property {
          ty: Type::String,
          optional: false,
          readonly: false,
        },
      ),
      (
        "another_field".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ]),
  });

  eq!(RenamedFields::shape(&ShapeOptions::Serialize), expected);
  eq!(RenamedFields::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn struct_with_skip_serializing() {
  #[derive(Shape)]
  #[allow(unused)]
  struct SkipSerializing {
    included: String,
    #[serde(skip_serializing)]
    skipped: i32,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "included".into(),
      Property {
        ty: Type::String,
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(SkipSerializing::shape(&ShapeOptions::Serialize), expected);
}

#[test]
fn struct_with_skip_deserializing() {
  #[derive(Shape)]
  #[allow(unused)]
  struct SkipDeserializing {
    included: String,
    #[serde(skip_deserializing)]
    skipped: i32,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "included".into(),
      Property {
        ty: Type::String,
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(
    SkipDeserializing::shape(&ShapeOptions::Deserialize),
    expected
  );
}

#[test]
fn struct_with_default() {
  #[derive(Shape)]
  #[allow(unused)]
  struct DefaultField {
    #[serde(default)]
    field_with_default: u32,
  }

  let expected_serialize = Type::Object(Object {
    properties: IndexMap::from([(
      "field_with_default".into(),
      Property {
        ty: Type::Number,
        optional: false,
        readonly: false,
      },
    )]),
  });

  let expected_deserialize = Type::Object(Object {
    properties: IndexMap::from([(
      "field_with_default".into(),
      Property {
        ty: Type::Number,
        optional: true,
        readonly: false,
      },
    )]),
  });

  eq!(
    DefaultField::shape(&ShapeOptions::Serialize),
    expected_serialize
  );
  eq!(
    DefaultField::shape(&ShapeOptions::Deserialize),
    expected_deserialize
  );
}

#[test]
fn struct_with_skip_serializing_if() {
  #[derive(Shape)]
  #[allow(unused)]
  struct SkipIfField {
    #[serde(skip_serializing_if = "something")]
    optional_field: String,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "optional_field".into(),
      Property {
        ty: Type::String,
        optional: true,
        readonly: false,
      },
    )]),
  });

  eq!(SkipIfField::shape(&ShapeOptions::Serialize), expected);
}

#[test]
fn struct_with_transparent() {
  #[derive(Shape)]
  #[serde(transparent)]
  #[allow(unused)]
  struct TransparentStruct {
    value: String,
  }

  let expected = Type::String;

  eq!(TransparentStruct::shape(&ShapeOptions::Serialize), expected);
  eq!(
    TransparentStruct::shape(&ShapeOptions::Deserialize),
    expected
  );
}

#[test]
fn enum_with_rename_all() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all = "PascalCase")]
  enum RenamedEnum {
    VariantOne,
    VariantTwo,
  }

  let expected = Type::Or(vec![
    Type::Literal(Literal::String("VariantOne".into())),
    Type::Literal(Literal::String("VariantTwo".into())),
  ]);

  eq!(RenamedEnum::shape(&ShapeOptions::Serialize), expected);
  eq!(RenamedEnum::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn struct_with_field_aliases() {
  #[derive(Shape)]
  #[allow(unused)]
  struct FieldAliases {
    #[serde(alias = "alt_name")]
    original_field: String,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "original_field".into(),
      Property {
        ty: Type::String,
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(FieldAliases::shape(&ShapeOptions::Serialize), expected);
  eq!(FieldAliases::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn struct_with_skip_and_rename() {
  #[derive(Shape)]
  #[allow(unused)]
  struct SkipAndRename {
    included: String,
    #[serde(skip)]
    #[serde(rename = "skipped_field")]
    skipped: i32,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "included".into(),
      Property {
        ty: Type::String,
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(SkipAndRename::shape(&ShapeOptions::Serialize), expected);
  eq!(SkipAndRename::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn enum_with_rename() {
  #[derive(Shape)]
  #[serde(rename_all = "UPPERCASE")]
  enum RenamedEnum {
    FirstVariant,
    SecondVariant,
  }

  let expected = Type::Or(vec![
    Type::Literal(Literal::String("FIRSTVARIANT".into())),
    Type::Literal(Literal::String("SECONDVARIANT".into())),
  ]);

  eq!(RenamedEnum::shape(&ShapeOptions::Serialize), expected);
  eq!(RenamedEnum::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn enum_with_skip_variant() {
  #[derive(Shape)]
  #[serde(tag = "type")]
  enum EnumWithSkip {
    Included,
    #[serde(skip)]
    Skipped,
  }

  let expected = Type::Or(vec![Type::Object(Object {
    properties: IndexMap::from([(
      "type".into(),
      Property {
        ty: Type::Literal(Literal::String("Included".into())),
        optional: false,
        readonly: false,
      },
    )]),
  })]);

  eq!(EnumWithSkip::shape(&ShapeOptions::Serialize), expected);
  eq!(EnumWithSkip::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn enum_with_skip_serializing_variant() {
  #[derive(Shape)]
  #[serde(tag = "type")]
  enum EnumSkipSerializing {
    AlwaysSerialized,
    #[serde(skip_serializing)]
    NeverSerialized,
  }

  let expected = Type::Or(vec![Type::Object(Object {
    properties: IndexMap::from([(
      "type".into(),
      Property {
        ty: Type::Literal(Literal::String("AlwaysSerialized".into())),
        optional: false,
        readonly: false,
      },
    )]),
  })]);

  eq!(
    EnumSkipSerializing::shape(&ShapeOptions::Serialize),
    expected
  );
}

#[test]
fn enum_with_skip_deserializing_variant() {
  #[derive(Shape)]
  #[serde(tag = "type")]
  enum EnumSkipDeserializing {
    AlwaysDeserialized,
    #[serde(skip_deserializing)]
    NeverDeserialized,
  }

  let expected = Type::Or(vec![Type::Object(Object {
    properties: IndexMap::from([(
      "type".into(),
      Property {
        ty: Type::Literal(Literal::String("AlwaysDeserialized".into())),
        optional: false,
        readonly: false,
      },
    )]),
  })]);

  eq!(
    EnumSkipDeserializing::shape(&ShapeOptions::Deserialize),
    expected
  );
}

#[test]
fn enum_with_internal_tagging() {
  #[derive(Shape)]
  #[serde(tag = "tag")]
  enum InternallyTaggedEnum {
    Unit,
    NewType(i32),
    Struct { field: String },
  }

  let expected = Type::Or(vec![
    // Unit variant
    Type::Object(Object {
      properties: IndexMap::from([(
        "tag".into(),
        Property {
          ty: Type::Literal(Literal::String("Unit".into())),
          optional: false,
          readonly: false,
        },
      )]),
    }),
    // Newtype variant
    Type::And(vec![
      Type::Object(Object {
        properties: IndexMap::from([(
          "tag".into(),
          Property {
            ty: Type::Literal(Literal::String("NewType".into())),
            optional: false,
            readonly: false,
          },
        )]),
      }),
      Type::Number,
    ]),
    // Struct variant
    Type::And(vec![
      Type::Object(Object {
        properties: IndexMap::from([(
          "tag".into(),
          Property {
            ty: Type::Literal(Literal::String("Struct".into())),
            optional: false,
            readonly: false,
          },
        )]),
      }),
      Type::Object(Object {
        properties: IndexMap::from([(
          "field".into(),
          Property {
            ty: Type::String,
            optional: false,
            readonly: false,
          },
        )]),
      }),
    ]),
  ]);

  eq!(
    InternallyTaggedEnum::shape(&ShapeOptions::Serialize),
    expected
  );
  eq!(
    InternallyTaggedEnum::shape(&ShapeOptions::Deserialize),
    expected
  );
}

#[test]
fn enum_with_adjacently_tagged() {
  #[derive(Shape)]
  #[serde(tag = "type", content = "data")]
  enum AdjacentlyTaggedEnum {
    VariantOne(i32),
    VariantTwo { field: String },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "type".into(),
          Property {
            ty: Type::Literal(Literal::String("VariantOne".into())),
            optional: false,
            readonly: false,
          },
        ),
        (
          "data".into(),
          Property {
            ty: Type::Number,
            optional: false,
            readonly: false,
          },
        ),
      ]),
    }),
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "type".into(),
          Property {
            ty: Type::Literal(Literal::String("VariantTwo".into())),
            optional: false,
            readonly: false,
          },
        ),
        (
          "data".into(),
          Property {
            ty: Type::Object(Object {
              properties: IndexMap::from([(
                "field".into(),
                Property {
                  ty: Type::String,
                  optional: false,
                  readonly: false,
                },
              )]),
            }),
            optional: false,
            readonly: false,
          },
        ),
      ]),
    }),
  ]);

  eq!(
    AdjacentlyTaggedEnum::shape(&ShapeOptions::Serialize),
    expected
  );
  eq!(
    AdjacentlyTaggedEnum::shape(&ShapeOptions::Deserialize),
    expected
  );
}

#[test]
fn enum_with_untagged() {
  #[derive(Shape)]
  #[serde(untagged)]
  enum UntaggedEnum {
    VariantOne(i32),
    VariantTwo(String),
  }

  let expected = Type::Or(vec![Type::Number, Type::String]);

  eq!(UntaggedEnum::shape(&ShapeOptions::Serialize), expected);
  eq!(UntaggedEnum::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn enum_with_flatten() {
  #[derive(Shape)]
  struct FlattenedStruct {
    field: String,
  }

  #[derive(Shape)]
  #[serde(untagged)]
  enum EnumWithFlatten {
    VariantA {
      #[serde(flatten)]
      flattened: FlattenedStruct,
    },
    VariantB {
      another_field: i32,
    },
  }

  let expected = Type::Or(vec![
    Type::And(vec![
      Type::Object(Object {
        properties: IndexMap::new(),
      }),
      Type::Object(Object {
        properties: IndexMap::from([(
          "field".into(),
          Property {
            ty: Type::String,
            optional: false,
            readonly: false,
          },
        )]),
      }),
    ]),
    Type::Object(Object {
      properties: IndexMap::from([(
        "another_field".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      )]),
    }),
  ]);

  eq!(EnumWithFlatten::shape(&ShapeOptions::Serialize), expected);
  eq!(EnumWithFlatten::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn enum_with_complex_variants() {
  #[derive(Shape)]
  #[allow(unused)]
  enum ComplexEnum {
    Unit,
    Tuple(u32, String),
    Struct { a: i32, b: bool },
  }

  let expected = Type::Or(vec![
    Type::Literal(Literal::String("Unit".into())),
    Type::Object(Object {
      properties: IndexMap::from([(
        "Tuple".into(),
        Property {
          ty: Type::Tuple(Tuple {
            items: vec![Type::Number, Type::String],
            rest: None,
          }),
          optional: false,
          readonly: false,
        },
      )]),
    }),
    Type::Object(Object {
      properties: IndexMap::from([(
        "Struct".into(),
        Property {
          optional: false,
          readonly: false,
          ty: Type::Object(Object {
            properties: IndexMap::from([
              (
                "a".into(),
                Property {
                  optional: false,
                  readonly: false,
                  ty: Type::Number,
                },
              ),
              (
                "b".into(),
                Property {
                  optional: false,
                  readonly: false,
                  ty: Type::Boolean,
                },
              ),
            ]),
          }),
        },
      )]),
    }),
  ]);

  eq!(ComplexEnum::shape(&ShapeOptions::Serialize), expected);
  eq!(ComplexEnum::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn option_type_serialization() {
  #[derive(Shape)]
  struct TestStruct {
    optional_field: Option<i32>,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "optional_field".into(),
      Property {
        ty: Type::Or(vec![Type::Number, Type::Null]),
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(TestStruct::shape(&ShapeOptions::Serialize), expected);
}

#[test]
fn option_type_deserialization() {
  #[derive(Shape)]
  struct TestStruct {
    optional_field: Option<i32>,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "optional_field".into(),
      Property {
        ty: Type::Or(vec![Type::Number, Type::Null, Type::Undefined]),
        optional: true,
        readonly: false,
      },
    )]),
  });

  eq!(TestStruct::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn nested_option_serialization() {
  #[derive(Shape)]
  #[allow(unused)]
  struct NestedStruct {
    nested_field: Option<Option<String>>,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "nested_field".into(),
      Property {
        ty: Type::Or(vec![Type::Or(vec![Type::String, Type::Null]), Type::Null]),
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(NestedStruct::shape(&ShapeOptions::Serialize), expected);
}

#[test]
fn nested_option_deserialization() {
  #[derive(Shape)]
  struct NestedStruct {
    nested_field: Option<Option<String>>,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "nested_field".into(),
      Property {
        ty: Type::Or(vec![
              Type::Or(vec![
                Type::String,
                Type::Null,
                Type::Undefined
              ]),
              Type::Null,
              Type::Undefined,
        ]),
        optional: true,
        readonly: false,
      },
    )]),
  });

  eq!(NestedStruct::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn option_in_enum_serialization() {
  #[derive(Shape)]
  #[serde(untagged)]
  #[allow(unused)]
  enum TestEnum {
    Variant { field: Option<u32> },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([(
        "field".into(),
        Property {
          ty: Type::Or(vec![Type::Number, Type::Null]),
          optional: false,
          readonly: false,
        },
      )])
    })
  ]);

  eq!(TestEnum::shape(&ShapeOptions::Serialize), expected);
}

#[test]
fn option_in_enum_deserialization() {
  #[derive(Shape)]
  #[serde(untagged)]
  enum TestEnum {
    Variant { field: Option<u32> },
  }

  let expected = Type::Or(vec![Type::Object(
    Object {
      properties: IndexMap::from([(
        "field".into(),
        Property {
          ty: Type::Or(vec![Type::Number, Type::Null, Type::Undefined]),
          optional: true,
          readonly: false,
        },
      )]),
    }
  )]);

  eq!(TestEnum::shape(&ShapeOptions::Deserialize), expected);
}

#[test]
fn option_in_vec_serialization() {
  #[derive(Shape)]
  struct VecOptionStruct {
    fields: Vec<Option<i64>>,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "fields".into(),
      Property {
        optional: false,
        readonly: false,
        ty: Type::Array(Array {
          item: Box::new(Type::Or(vec![Type::Number, Type::Null])),   
        }),
      }
    )])
  });

  eq!(VecOptionStruct::shape(&ShapeOptions::Serialize), expected);
}

#[test]
fn option_in_vec_deserialization() {
  #[derive(Shape)]
  struct VecOptionStruct {
    fields: Vec<Option<i64>>,
  }

  let expected = Type::Object(Object {
    properties: IndexMap::from([(
      "fields".into(),
      Property {
        optional: false,
        readonly: false,
        ty: Type::Array(Array {
          item: Box::new(Type::Or(vec![
            Type::Number,
            Type::Null,
            Type::Undefined,
          ])),
        })
      },
    )]),
  });

  eq!(VecOptionStruct::shape(&ShapeOptions::Deserialize), expected);
}
