use shape::{Array, Literal, Object, Property, Record, Shape, ShapeOptions, Type};
use shape::ToTypescript;
use text_diff::print_diff;

fn simplify(ty: &str) -> String {
  let re = regex_static::static_regex!(r"\s+");
  re.replace_all(ty, "").trim().to_string()
}

macro_rules! eq {
  ($a:expr, $b:expr) => {
    let a = simplify(&$a.to_typescript());
    let b = simplify(&$b);

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
  eq!(String::shape(&ShapeOptions::Serialize), "string");
  eq!(String::shape(&ShapeOptions::Deserialize), "string");

  eq!(u8::shape(&ShapeOptions::Serialize), "number");
  eq!(u8::shape(&ShapeOptions::Deserialize), "number");

  eq!(u16::shape(&ShapeOptions::Serialize), "number");
  eq!(u16::shape(&ShapeOptions::Deserialize), "number");

  eq!(u32::shape(&ShapeOptions::Serialize), "number");
  eq!(u32::shape(&ShapeOptions::Deserialize), "number");

  eq!(u64::shape(&ShapeOptions::Serialize), "number");
  eq!(u64::shape(&ShapeOptions::Deserialize), "number");

  eq!(u128::shape(&ShapeOptions::Serialize), "number");
  eq!(u128::shape(&ShapeOptions::Deserialize), "number");

  eq!(usize::shape(&ShapeOptions::Serialize), "number");
  eq!(usize::shape(&ShapeOptions::Deserialize), "number");

  eq!(i8::shape(&ShapeOptions::Serialize), "number");
  eq!(i8::shape(&ShapeOptions::Deserialize), "number");

  eq!(i16::shape(&ShapeOptions::Serialize), "number");
  eq!(i16::shape(&ShapeOptions::Deserialize), "number");

  eq!(i32::shape(&ShapeOptions::Serialize), "number");
  eq!(i32::shape(&ShapeOptions::Deserialize), "number");

  eq!(i64::shape(&ShapeOptions::Serialize), "number");
  eq!(i64::shape(&ShapeOptions::Deserialize), "number");

  eq!(i128::shape(&ShapeOptions::Serialize), "number");
  eq!(i128::shape(&ShapeOptions::Deserialize), "number");

  eq!(isize::shape(&ShapeOptions::Serialize), "number");
  eq!(isize::shape(&ShapeOptions::Deserialize), "number");

  eq!(f32::shape(&ShapeOptions::Serialize), "number");
  eq!(f32::shape(&ShapeOptions::Deserialize), "number");

  eq!(f64::shape(&ShapeOptions::Serialize), "number");
  eq!(f64::shape(&ShapeOptions::Deserialize), "number");

  eq!(bool::shape(&ShapeOptions::Serialize), "boolean");
  eq!(bool::shape(&ShapeOptions::Deserialize), "boolean");

  eq!(<()>::shape(&ShapeOptions::Serialize), "null");
  eq!(<()>::shape(&ShapeOptions::Deserialize), "null");
}

#[test]
fn option() {
  eq!(Option::<String>::shape(&ShapeOptions::Serialize), "(string|null)");
  eq!(Option::<String>::shape(&ShapeOptions::Deserialize), "(string|null|undefined)");
}

#[test]
fn array() {
  eq!(Type::Array(Array { item: Box::new(Type::String) }), "Array<string>");
}

#[test]
fn tuple() {
  eq!(<(String, i32)>::shape(&ShapeOptions::Serialize), "[string,number]");
  eq!(<(String, i32)>::shape(&ShapeOptions::Deserialize), "[string,number]");
}

#[test]
fn record() {
  eq!(Type::Record(Record { key: Box::new(Type::String), value: Box::new(Type::Number) }), "{[key:string]:number}");
}

#[test]
fn object() {
  let shape = Type::Object(
    Object{
      properties: indexmap::IndexMap::from([
        (
          "a".into(), 
          Property { 
            ty: Type::String,
            optional: false,
            readonly: false
          }
        ),
        (
          "b".into(), 
          Property { 
            ty: Type::Number,
            optional: false,
            readonly: false
          }
        )
      ])
    }
  );

  eq!(shape, "{a:string;b:number;}");
}

#[test]
fn logical_or_and() {
  let shape = Type::Or(vec![
    Type::String,
    Type::Number,
    Type::And(vec![
      Type::String,
      Type::Boolean,
    ])
  ]);

  eq!(shape, "(string|number|(string&boolean))");
}

#[test]
fn literals() {
  eq!(Type::Literal(Literal::String("a".into())), "\"a\"");
  eq!(Type::Literal(Literal::Number(1.0)), "1");
  eq!(Type::Literal(Literal::Boolean(true)), "true");
  eq!(Type::Literal(Literal::Boolean(false)), "false");
}

#[test]
fn never() {
  eq!(Type::Never, "never");
}

#[test]
fn custom() {
  eq!(Type::Custom("custom".into()), "custom");
}

#[test]
fn quoted() {
  let shape = Type::Object(Object {
    properties: indexmap::IndexMap::from([(
      "quoted-key".into(),
      Property {
        ty: Type::String,
        optional: false,
        readonly: false,
      },
    ), (
      "2two".into(),
      Property {
        ty: Type::Number,
        optional: true,
        readonly: true,
      },
    ), (
      "".into(),
      Property {
        ty: Type::Boolean,
        optional: false,
        readonly: false,
      },
    )]),
  });

  eq!(shape, "{\"quoted-key\":string;readonly \"2two\"?:number;\"\":boolean;}");
}

#[test]
fn readonly() {
  let shape = Type::Object(Object {
    properties: indexmap::IndexMap::from([(
      "key".into(),
      Property {
        ty: Type::String,
        optional: false,
        readonly: true,
      },
    )]),
  });

  eq!(shape, "{readonly key:string;}");
}

#[test]
fn optional() {
  let shape = Type::Object(Object {
    properties: indexmap::IndexMap::from([(
      "key".into(),
      Property {
        ty: Type::String,
        optional: true,
        readonly: false,
      },
    )]),
  });

  eq!(shape, "{key?:string;}");
}