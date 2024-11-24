use indexmap::IndexMap;
use shape::{Literal, Object, Property, Shape, ShapeOptions, Type};

mod common;

#[test]
fn rename_all_ser_de() {
  #[derive(Shape)]
  #[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
  #[allow(unused)]
  struct RenamedFields { 
    original_field: String,
    another_field: i32,
  }

  let ser = Type::Object(Object {
    properties: IndexMap::from([
      (
        "originalField".into(),
        Property {
          ty: Type::String,
          optional: false,
          readonly: false,
        },
      ),
      (
        "anotherField".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ]),
  });

  let de = Type::Object(Object {
    properties: IndexMap::from([
      (
        "OriginalField".into(),
        Property {
          ty: Type::String,
          optional: false,
          readonly: false,
        },
      ),
      (
        "AnotherField".into(),
        Property {
          ty: Type::Number,
          optional: false,
          readonly: false,
        },
      ),
    ]),
  });

  eq!(RenamedFields::shape(&ShapeOptions::Serialize), ser);
  eq!(RenamedFields::shape(&ShapeOptions::Deserialize), de);
}

#[test]
fn rename_ser_de() {
  #[derive(Shape)]
  #[allow(unused)]
  struct RenamedFields { 
    #[serde(rename(serialize = "_field", deserialize = "Field"))]
    field: String,
  }

  let ser = Type::Object(Object {
    properties: IndexMap::from([(
      "_field".into(),
      Property {
        ty: Type::String,
        optional: false,
        readonly: false,
      },
    )]),
  });

  let de = Type::Object(Object {
    properties: IndexMap::from([(
      "Field".into(),
      Property {
        ty: Type::String,
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(RenamedFields::shape(&ShapeOptions::Serialize), ser);
  eq!(RenamedFields::shape(&ShapeOptions::Deserialize), de);
}

#[test]
fn enum_rename_all_fields() {
  #[derive(Shape)]
  #[serde(rename_all_fields(serialize = "SCREAMING_SNAKE_CASE", deserialize = "camelCase"))]
  #[allow(unused)]
  enum RenamedEnum {
    VariantOne {
      some_field: String
    },
  }

  let ser = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "VariantOne".into(),
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
          }
        )
      ]),
    })
  ]);

  let de = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "VariantOne".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "someField".into(),
                  Property {
                    ty: Type::String,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            }),
          }
        )
      ]),
    })
  ]);

  eq!(RenamedEnum::shape(&ShapeOptions::Serialize), ser);
  eq!(RenamedEnum::shape(&ShapeOptions::Deserialize), de);
}

#[test]
fn enum_variant_rename() {
  #[derive(Shape)]
  #[allow(unused)]
  enum RenamedEnum {
    #[serde(rename(serialize = "variant_one", deserialize = "VARIANT-ONE"))]
    VariantOne,
  }

  let ser = Type::Or(vec![
    Type::Literal(Literal::String("variant_one".into())),
  ]);
  
  let de = Type::Or(vec![
    Type::Literal(Literal::String("VARIANT-ONE".into())),
  ]);

  eq!(RenamedEnum::shape(&ShapeOptions::Serialize), ser);
  eq!(RenamedEnum::shape(&ShapeOptions::Deserialize), de);
}

#[test]
fn enum_variant_rename_all() {
  #[derive(Shape)]
  #[allow(unused)]
  enum RenamedEnum {
    #[serde(rename_all(serialize = "PascalCase", deserialize = "SCREAMING-KEBAB-CASE"))]
    VariantOne {
      some_field: String
    },
  }

  let ser = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "VariantOne".into(),
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
          }
        )
      ]),
    })
  ]);

  let de = Type::Or(vec![
    Type::Object(Object {
      properties: IndexMap::from([
        (
          "VariantOne".into(),
          Property {
            optional: false,
            readonly: false,
            ty: Type::Object(Object {
              properties: IndexMap::from([
                (
                  "SOME-FIELD".into(),
                  Property {
                    ty: Type::String,
                    optional: false,
                    readonly: false,
                  },
                ),
              ]),
            }),
          }  
        ),
      ])
    })   
  ]);

  eq!(RenamedEnum::shape(&ShapeOptions::Serialize), ser);
  eq!(RenamedEnum::shape(&ShapeOptions::Deserialize), de);
}