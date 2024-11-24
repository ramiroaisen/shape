use indexmap::IndexMap;
use shape::{Literal, Object, Property, ShapeOptions, Type};
use shape::Shape;

mod common;

// snake_case
#[test]
fn snake_upper() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all = "snake_case")]
  enum Snake {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "some_variant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
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
            }),
          },
        ),
      ]),
    })
  ]);

  eq!(Snake::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Snake::shape(&ShapeOptions::for_deserialize()), expected); 
}

// SCREAMING_SNAKE_CASE
#[test]
fn screaming_snake_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
  enum ScreamingSnake {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SOME_VARIANT".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
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
            }),
          },
        ),
      ])
    })
  ]);

  eq!(ScreamingSnake::shape(&ShapeOptions::for_serialize()), expected);
  eq!(ScreamingSnake::shape(&ShapeOptions::for_deserialize()), expected); 
}

// lowercase
#[test]
fn lowercase() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all = "lowercase")]
  enum Lowercase {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "somevariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
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
            }),
          },
        ),
      ]),
    })
  ]);

  eq!(Lowercase::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Lowercase::shape(&ShapeOptions::for_deserialize()), expected); 
}

// camelCase
#[test]
fn camel_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all = "camelCase")]
  enum Camel {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "someVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
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
            }),
          },
        ),
      ]),
    })
  ]);

  eq!(Camel::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Camel::shape(&ShapeOptions::for_deserialize()), expected); 
}

// PascalCase
#[test]
fn pascal_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all = "PascalCase")]
  enum Pascal {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SomeVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
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
            }),       
          },
        ),
      ]),
    })
  ]);

  eq!(Pascal::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Pascal::shape(&ShapeOptions::for_deserialize()), expected); 
}

// SCREAMING-KEBAB-CASE
#[test]
fn screaming_kebab_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all = "SCREAMING-KEBAB-CASE")]
  enum ScreamingKebab {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SOME-VARIANT".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
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
            }),
          },
        ),
      ]),
    })
  ]);

  eq!(ScreamingKebab::shape(&ShapeOptions::for_serialize()), expected);
  eq!(ScreamingKebab::shape(&ShapeOptions::for_deserialize()), expected); 
}

// kebab-case
#[test]
fn kebab_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all = "kebab-case")]
  enum Kebab {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "some-variant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
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
            }),
          },
        ),
      ])
    })
  ]);

  eq!(Kebab::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Kebab::shape(&ShapeOptions::for_deserialize()), expected); 
}

// UPPERCASE
#[test]
fn uppercase() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all = "UPPERCASE")]
  enum Uppercase {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SOMEVARIANT".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
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
            }),
          },      
        ),
      ])
    })
  ]);

  eq!(Uppercase::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Uppercase::shape(&ShapeOptions::for_deserialize()), expected); 
}

// rename_all_fields
// snake_case
#[test]
fn rename_all_fields_snake_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all_fields = "snake_case")]
  #[allow(non_snake_case)]
  enum Snake {
    SomeVariant {
      someField: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SomeVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
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
            })
          }
        ),
      ])
    })
  ]);

  eq!(Snake::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Snake::shape(&ShapeOptions::for_deserialize()), expected); 
}

// SCREAMING_SNAKE_CASE
#[test]
fn rename_all_fields_screaming_snake_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all_fields = "SCREAMING_SNAKE_CASE")]
  #[allow(non_snake_case)]
  enum ScreamingSnake {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SomeVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "SOME_FIELD".into(),
                  Property {
                    ty: Type::Number,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            })
          }  
        ),
      ])
    })   
  ]);

  eq!(ScreamingSnake::shape(&ShapeOptions::for_serialize()), expected);
  eq!(ScreamingSnake::shape(&ShapeOptions::for_deserialize()), expected); 
}

// lowercase
#[test]
fn rename_all_fields_lowercase() {
  #[derive(Shape)]
  #[allow(unused)]
  #[allow(non_snake_case)]
  #[serde(rename_all_fields = "lowercase")]
  enum Lowercase {
    SomeVariant {
      someField: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SomeVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "somefield".into(),
                  Property {
                    ty: Type::Number,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            }),
            
          },
        ),
      ])
    })
  ]);

  eq!(Lowercase::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Lowercase::shape(&ShapeOptions::for_deserialize()), expected); 
}

// UPPERCASE
#[test]
fn rename_all_fields_uppercase() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all_fields = "UPPERCASE")]
  #[allow(non_snake_case)]
  enum Uppercase {
    SomeVariant {
      someField: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SomeVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "SOMEFIELD".into(),
                  Property {
                    ty: Type::Number,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            }),
          },
        ),
      ])
    })
  ]);

  eq!(Uppercase::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Uppercase::shape(&ShapeOptions::for_deserialize()), expected); 
}

// camelCase
#[test]
fn rename_all_fields_camel_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all_fields = "camelCase")]
  enum Camel {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SomeVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "someField".into(),
                  Property {
                    ty: Type::Number,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            }),
            
          },
        ),
      ])
    })
  ]);

  eq!(Camel::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Camel::shape(&ShapeOptions::for_deserialize()), expected); 
}

// SCREAMING-KEBAB-CASE
#[test]
fn rename_all_fields_screaming_kebab_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all_fields(serialize = "SCREAMING-KEBAB-CASE", deserialize = "camelCase"))]
  #[allow(non_snake_case)]
  enum ScreamingKebab {
    SomeVariant {
      some_field: u8,
    },
  }

  let ser = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SomeVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "SOME-FIELD".into(),
                  Property {
                    ty: Type::Number,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            }),
          },
        ),
      ])
    })
  ]);

  let de = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SomeVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "someField".into(),
                  Property {
                    ty: Type::Number,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            }),
          },
        ),
      ])
    })
  ]);

  eq!(ScreamingKebab::shape(&ShapeOptions::for_serialize()), ser);
  eq!(ScreamingKebab::shape(&ShapeOptions::for_deserialize()), de); 
}

// kebab-case
#[test]
fn rename_all_fields_kebab_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all_fields = "kebab-case")]
  #[allow(non_snake_case)]
  enum Kebab {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SomeVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "some-field".into(),
                  Property {
                    ty: Type::Number,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            }),
          },
        ),
      ])
    })
  ]);

  eq!(Kebab::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Kebab::shape(&ShapeOptions::for_deserialize()), expected); 
}

// PascalCase
#[test]
fn rename_all_fields_pascal_case() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all_fields = "PascalCase")]
  #[allow(non_snake_case)]
  enum Pascal {
    SomeVariant {
      some_field: u8,
    },
  }

  let expected = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "SomeVariant".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "SomeField".into(),
                  Property {
                    ty: Type::Number,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            }),
          },
        ),
      ])
    })
  ]);

  eq!(Pascal::shape(&ShapeOptions::for_serialize()), expected);
  eq!(Pascal::shape(&ShapeOptions::for_deserialize()), expected); 
}

#[test]
fn variant_rename_all() {
  #[derive(Shape)]
  #[allow(unused)]
  #[serde(rename_all(serialize = "camelCase", deserialize = "SCREAMING-KEBAB-CASE"))]
  enum RenamedEnum {
    #[serde(rename_all(serialize = "PascalCase", deserialize = "UPPERCASE"))]
    VariantOne {
      some_field: String
    },
  }

  let ser = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "variantOne".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "SomeField".into(),
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
      ])
    })
  ]);

  let de = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "VARIANT-ONE".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "SOME_FIELD".into(),
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
      ])
    })
  ]);

  eq!(RenamedEnum::shape(&ShapeOptions::for_serialize()), ser); 
  eq!(RenamedEnum::shape(&ShapeOptions::for_deserialize()), de);
}

#[test]
fn variant_rename() {
  #[derive(Shape)]
  #[allow(unused)]
  enum RenamedEnum {
    #[serde(rename(serialize = "variant_one", deserialize = "VARIANT-ONE"))]
    VariantOne,
    #[serde(rename = "VARI_TWO")]
    VariantTwo,
  }

  let ser = Type::Or(vec![
    Type::Literal(Literal::String("variant_one".into())),
    Type::Literal(Literal::String("VARI_TWO".into())),
  ]);
  
  let de = Type::Or(vec![
    Type::Literal(Literal::String("VARIANT-ONE".into())),
    Type::Literal(Literal::String("VARI_TWO".into())),
  ]);

  eq!(RenamedEnum::shape(&ShapeOptions::for_serialize()), ser);
  eq!(RenamedEnum::shape(&ShapeOptions::for_deserialize()), de);
}