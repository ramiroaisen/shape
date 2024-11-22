use indexmap::IndexMap;
use shape::{Object, Property, Shape, ShapeOptions, Type};

#[test]
fn primitives() {
  for options in [ShapeOptions::Serialize, ShapeOptions::Deserialize] {
    assert_eq!(Type::String, String::shape(&options));
    assert_eq!(Type::String, str::shape(&options));
    assert_eq!(Type::Number, u8::shape(&options));
    assert_eq!(Type::Number, u16::shape(&options));
    assert_eq!(Type::Number, u32::shape(&options));
    assert_eq!(Type::Number, u64::shape(&options));
    assert_eq!(Type::Number, u128::shape(&options));
    assert_eq!(Type::Number, usize::shape(&options));
    assert_eq!(Type::Number, i8::shape(&options));
    assert_eq!(Type::Number, i16::shape(&options));
    assert_eq!(Type::Number, i32::shape(&options));
    assert_eq!(Type::Number, i64::shape(&options));
    assert_eq!(Type::Number, i128::shape(&options));
    assert_eq!(Type::Number, isize::shape(&options));
    assert_eq!(Type::Number, f32::shape(&options));
    assert_eq!(Type::Number, f64::shape(&options));
    assert_eq!(Type::Boolean, bool::shape(&options));
    assert_eq!(Type::Null, <()>::shape(&options));
  }
}

#[test]
fn refs() {
  for options in [ShapeOptions::Serialize, ShapeOptions::Deserialize] {
    assert_eq!(Type::String, <&String>::shape(&options));
    assert_eq!(Type::String, <&str>::shape(&options));
    assert_eq!(Type::Number, <&u8>::shape(&options));
    assert_eq!(Type::Number, <&u16>::shape(&options));
    assert_eq!(Type::Number, <&u32>::shape(&options));
    assert_eq!(Type::Number, <&u64>::shape(&options));
    assert_eq!(Type::Number, <&u128>::shape(&options));
    assert_eq!(Type::Number, <&usize>::shape(&options));
    assert_eq!(Type::Number, <&i8>::shape(&options));
    assert_eq!(Type::Number, <&i16>::shape(&options));
    assert_eq!(Type::Number, <&i32>::shape(&options));
    assert_eq!(Type::Number, <&i64>::shape(&options));
    assert_eq!(Type::Number, <&i128>::shape(&options));
    assert_eq!(Type::Number, <&isize>::shape(&options));
    assert_eq!(Type::Number, <&f32>::shape(&options));
    assert_eq!(Type::Number, <&f64>::shape(&options));
    assert_eq!(Type::Boolean, <&bool>::shape(&options));
    assert_eq!(Type::Null, <&()>::shape(&options));
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

  assert_eq!(Simple::shape(&ShapeOptions::Serialize), expected);
  assert_eq!(Simple::shape(&ShapeOptions::Deserialize), expected);
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
                }
            ),
            (
                "opt".into(),
                Property {
                    ty: Type::Boolean,
                    optional: true,
                    readonly: false,
                }
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
                                }
                            ),
                            (
                                "field-two".into(),
                                Property {
                                    ty: Type::Number,
                                    optional: false,
                                    readonly: false,
                                }
                            ),
                        ]),
                    }),
                    optional: false,
                    readonly: false,
                }
            ),
        ]),
    });

    assert_eq!(SerdeTest::shape(&ShapeOptions::Serialize), expected);
    assert_eq!(SerdeTest::shape(&ShapeOptions::Deserialize), expected);
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
        properties: IndexMap::from([
          (
            "top_level".into(),
            Property {
              ty: Type::String,
              optional: false,
              readonly: false,
            },
          )
        ])
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
            }
          )
        ])
      })
    ]);

    assert_eq!(Wrapper::shape(&ShapeOptions::Serialize), expected);
    assert_eq!(Wrapper::shape(&ShapeOptions::Deserialize), expected);
}


#[test]
fn newtype() {
  #[derive(Shape)]
  struct Newtype(pub i32);

  let expected = Type::Number;
  assert_eq!(Newtype::shape(&ShapeOptions::Serialize), expected);
  assert_eq!(Newtype::shape(&ShapeOptions::Deserialize), expected);
}