mod common;

use indexmap::indexmap;
use serde_json::json;
use shape::{Array, IsAsignable, Literal, Object, Property, Record, Tuple, Type};

#[test]
fn primitives() {
  assert!(Type::String.is_assignable(&json!("a")));
  assert!(Type::Number.is_assignable(&json!(1)));
  assert!(Type::Boolean.is_assignable(&json!(true)));
  assert!(Type::Null.is_assignable(&json!(null)));

  assert!(!Type::String.is_assignable(&json!(1)));
  assert!(!Type::Number.is_assignable(&json!("a")));
  assert!(!Type::Boolean.is_assignable(&json!(1)));
  assert!(!Type::Null.is_assignable(&json!(true)));
  assert!(!Type::Undefined.is_assignable(&json!(null)));
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn literals() {
  assert!(Type::Literal(Literal::String("a".into())).is_assignable(&json!("a")));
  assert!(Type::Literal(Literal::Number(1.0)).is_assignable(&json!(1)));
  assert!(Type::Literal(Literal::Boolean(true)).is_assignable(&json!(true)));

  assert!(!Type::Literal(Literal::String("a".into())).is_assignable(&json!(1)));
  assert!(!Type::Literal(Literal::String("a".into())).is_assignable(&json!("b")));
  assert!(!Type::Literal(Literal::Number(1.0)).is_assignable(&json!(1.1)));
  assert!(!Type::Literal(Literal::Boolean(true)).is_assignable(&json!(false)));
}

#[test]
fn never() {
  assert!(!Type::Never.is_assignable(&json!(null)));
  assert!(!Type::Never.is_assignable(&json!(1)));
  assert!(!Type::Never.is_assignable(&json!(true)));
}

#[test]
fn tuple() {
  assert!(Type::Tuple(Tuple {
    items: vec![Type::String, Type::Number],
    rest: None,
  })
  .is_assignable(&json!(["a", 1])));

  assert!(Type::Tuple(Tuple {
    items: vec![Type::String, Type::Number],
    rest: Some(Box::new(Type::Boolean)),
  })
  .is_assignable(&json!(["a", 1, true, false])));

  assert!(Type::Tuple(Tuple {
    items: vec![Type::Literal(Literal::String("a".into())), Type::Number],
    rest: Some(Box::new(Type::Boolean)),
  })
  .is_assignable(&json!(["a", 1, true, false])));

  assert!(!Type::Tuple(Tuple {
    items: vec![Type::Literal(Literal::String("a".into())), Type::Number],
    rest: Some(Box::new(Type::Boolean)),
  })
  .is_assignable(&json!(["b", 1, true, false])));

  assert!(!Type::Tuple(Tuple {
    items: vec![Type::String, Type::Number],
    rest: Some(Box::new(Type::Boolean)),
  })
  .is_assignable(&json!(["a", 1, 2, false])));

  assert!(!Type::Tuple(Tuple {
    items: vec![Type::String, Type::Number],
    rest: None,
  })
  .is_assignable(&json!([1])));

  assert!(!Type::Tuple(Tuple {
    items: vec![],
    rest: None,
  }).is_assignable(&json!(true)));
}

#[test]
fn objects() {
  // Caso 1: Objeto con propiedades obligatorias.
  let obj_type = Type::Object(Object {
    properties: indexmap! {
        "name".to_string() => Property { ty: Type::String, optional: false, readonly: false },
        "age".to_string() => Property { ty: Type::Number, optional: false, readonly: false },
    },
  });
  assert!(obj_type.is_assignable(&json!({ "name": "Alice", "age": 30 })));
  assert!(!obj_type.is_assignable(&json!({ "name": "Alice" }))); // Falta "age".

  // Caso 2: Propiedad opcional.
  let obj_type = Type::Object(Object {
    properties: indexmap! {
        "name".to_string() => Property { ty: Type::String, optional: false, readonly: false },
        "age".to_string() => Property { ty: Type::Number, optional: true, readonly: false },
    },
  });
  assert!(obj_type.is_assignable(&json!({ "name": "Alice" }))); // "age" opcional.
  assert!(obj_type.is_assignable(&json!({ "name": "Alice", "age": 30 })));

  // Caso 3: Propiedad de solo lectura.
  let obj_type = Type::Object(Object {
    properties: indexmap! {
        "id".to_string() => Property { ty: Type::Number, optional: false, readonly: true },
    },
  });
  assert!(obj_type.is_assignable(&json!({ "id": 42 }))); // "readonly" no afecta asignabilidad.

  // Caso 4: Propiedad con un tipo compuesto.
  let obj_type = Type::Object(Object {
    properties: indexmap! {
        "data".to_string() => Property { ty: Type::Array(Array { item: Box::new(Type::String) }), optional: false, readonly: false },
    },
  });
  assert!(obj_type.is_assignable(&json!({ "data": ["a", "b", "c"] })));
  assert!(!obj_type.is_assignable(&json!({ "data": ["a", 1, "c"] }))); // Tipo incorrecto en el array.

  assert!(!obj_type.is_assignable(&json!([])));
  assert!(!obj_type.is_assignable(&json!(true)));
  assert!(!obj_type.is_assignable(&json!(null)));
}

#[test]
fn arrays() {
  // Caso 5: Array de un tipo simple.
  let array_type = Type::Array(Array {
    item: Box::new(Type::Number),
  });
  assert!(array_type.is_assignable(&json!([1, 2, 3])));
  assert!(!array_type.is_assignable(&json!([1, "a", 3]))); // Tipo incorrecto.

  // Caso 6: Array vacío.
  let array_type = Type::Array(Array {
    item: Box::new(Type::String),
  });
  assert!(array_type.is_assignable(&json!([]))); // Un array vacío debería ser asignable.

  // Caso 7: Array con tipos literales.
  let array_type = Type::Array(Array {
    item: Box::new(Type::Literal(Literal::String("a".into()))),
  });
  assert!(array_type.is_assignable(&json!(["a", "a", "a"])));
  assert!(!array_type.is_assignable(&json!(["a", "b"]))); // Elemento incorrecto.

  assert!(!Type::Array(Array {
    item: Box::new(Type::String),
  }).is_assignable(&json!({ "a": 1 })))
}

#[test]
fn records() {
  // Caso 8: Record con claves y valores de tipo específico.
  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::String),
    value: Box::new(Type::Number),
  });
  assert!(record_type.is_assignable(&json!({ "a": 1, "b": 2 })));
  assert!(!record_type.is_assignable(&json!({ "a": "1", "b": 2 }))); // Valor incorrecto.

  // Caso 9: Record opcional.
  let record_type = Type::Record(Record {
    optional: true,
    readonly: false,
    key: Box::new(Type::String),
    value: Box::new(Type::Boolean),
  });
  assert!(record_type.is_assignable(&json!({}))); // Record vacío permitido.
  assert!(record_type.is_assignable(&json!({ "a": true, "b": false })));

  // Caso 10: Record con claves literales y valores variados.
  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Literal(Literal::String("key".into()))),
    value: Box::new(Type::Or(vec![Type::Number, Type::String])),
  });
  assert!(record_type.is_assignable(&json!({ "key": 42 })));
  assert!(record_type.is_assignable(&json!({ "key": "value" })));
  assert!(!record_type.is_assignable(&json!({ "key": true }))); // Valor incorrecto.

  // record number
  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Number),
    value: Box::new(Type::Number),
  });
  assert!(record_type.is_assignable(&json!({ "1": 42 })));
  assert!(record_type.is_assignable(&json!({ "3.0": 42.0 })));
  assert!(!record_type.is_assignable(&json!({ "key": "value" })));

  // record number
  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Or( vec![ Type::Literal(Literal::Number(5.0)), Type::Literal(Literal::String("30".into())) ])),
    value: Box::new(Type::Number),
  });
  assert!(record_type.is_assignable(&json!({ "30": 42, "5": 1 })));
  assert!(!record_type.is_assignable(&json!({ "5": 42.0 })));
  assert!(!record_type.is_assignable(&json!({ "key": "value" }))); 

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::And( vec![ Type::Literal(Literal::String("a".into())), Type::Literal(Literal::String("b".into())) ])),
    value: Box::new(Type::Number),
  });
  assert!(record_type.is_assignable(&json!({})));
  assert!(record_type.is_assignable(&json!({ "a": 42 })));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::And( vec![ Type::Literal(Literal::String("a".into())), Type::Literal(Literal::String("a".into())) ])),
    value: Box::new(Type::Number),
  });
  assert!(record_type.is_assignable(&json!({ "a": 42 })));
  assert!(!record_type.is_assignable(&json!({ "a": "b" })));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::String),
    value: Box::new(Type::Never),
  });
  assert!(record_type.is_assignable(&json!({})));
  assert!(!record_type.is_assignable(&json!({ "a": "b" })));


  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Custom("a".into())),
    value: Box::new(Type::Never),
  });
  assert!(!record_type.is_assignable(&json!({})));


  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Null),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!({})));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Undefined),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!({})));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Array(Array { item: Box::new(Type::String) })),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!({})));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Object(Object { properties: indexmap! {} })),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!({})));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Never),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!({})));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Boolean),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!({})));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Record(Record{ optional: false, readonly: false, key: Box::new(Type::String), value: Box::new(Type::String) })),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!({})));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Tuple(Tuple{ items: vec![Type::String, Type::Number], rest: None })),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!({})));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Literal(Literal::Boolean(true))),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!({})));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::Literal(Literal::Boolean(true))),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!({})));

  let record_type = Type::Record(Record {
    optional: false,
    readonly: false,
    key: Box::new(Type::String),
    value: Box::new(Type::String),
  });
  assert!(!record_type.is_assignable(&json!([])));
}


#[test]
fn and_type() {
  // Caso 1: Un valor que es tanto un número como un literal específico.
  let and_type = Type::And(vec![Type::Number, Type::Literal(Literal::Number(42.0))]);
  assert!(and_type.is_assignable(&json!(42)));
  assert!(!and_type.is_assignable(&json!(43))); // No es el literal correcto.
  assert!(!and_type.is_assignable(&json!("42"))); // No es un número.

  // Caso 2: Objeto con una propiedad específica y un tipo compuesto.
  let and_type = Type::And(vec![
    Type::Object(Object {
      properties: indexmap! {
          "key".to_string() => Property { ty: Type::String, optional: false, readonly: false },
      },
    }),
    Type::Object(Object {
      properties: indexmap! {
          "value".to_string() => Property { ty: Type::Number, optional: false, readonly: false },
      },
    }),
  ]);
  assert!(and_type.is_assignable(&json!({ "key": "test", "value": 123 })));
  assert!(!and_type.is_assignable(&json!({ "key": "test" }))); // Falta "value".
  assert!(!and_type.is_assignable(&json!({ "value": 123 }))); // Falta "key".

  // Caso 3: Un array que debe cumplir dos condiciones.
  let and_type = Type::And(vec![
    Type::Array(Array {
      item: Box::new(Type::String),
    }),
    Type::Array(Array {
      item: Box::new(Type::Literal(Literal::String("test".into()))),
    }),
  ]);
  assert!(and_type.is_assignable(&json!(["test", "test"])));
  assert!(!and_type.is_assignable(&json!(["test", "other"]))); // No cumple con los literales.
}

#[test]
fn or_type() {
  // Caso 4: Un valor que puede ser un número o una cadena.
  let or_type = Type::Or(vec![Type::Number, Type::String]);
  assert!(or_type.is_assignable(&json!(42)));
  assert!(or_type.is_assignable(&json!("hello")));
  assert!(!or_type.is_assignable(&json!(true))); // No es número ni cadena.

  // Caso 5: Un valor que puede ser un literal o un tipo más general.
  let or_type = Type::Or(vec![
    Type::Literal(Literal::String("specific".into())),
    Type::String,
  ]);
  assert!(or_type.is_assignable(&json!("specific"))); // Cumple con el literal.
  assert!(or_type.is_assignable(&json!("general"))); // Cumple con el tipo general.
  assert!(!or_type.is_assignable(&json!(123))); // No es una cadena.

  // Caso 6: Objeto con una propiedad opcional o clave específica.
  let or_type = Type::Or(vec![
    Type::Object(Object {
      properties: indexmap! {
          "optionalKey".to_string() => Property { ty: Type::String, optional: false, readonly: false },
      },
    }),
    Type::Object(Object {
      properties: indexmap! {
          "requiredKey".to_string() => Property { ty: Type::Number, optional: false, readonly: false },
      },
    }),
  ]);
  assert!(or_type.is_assignable(&json!({ "optionalKey": "value" }))); // Cumple con el primer tipo.
  assert!(or_type.is_assignable(&json!({ "requiredKey": 42 }))); // Cumple con el segundo tipo.
  assert!(!or_type.is_assignable(&json!({ "anotherKey": true }))); // No cumple con ninguno.

  // Caso 7: Array que puede contener números o cadenas.
  let or_type = Type::Or(vec![
    Type::Array(Array {
      item: Box::new(Type::Number),
    }),
    Type::Array(Array {
      item: Box::new(Type::String),
    }),
  ]);
  assert!(or_type.is_assignable(&json!([1, 2, 3])));
  assert!(or_type.is_assignable(&json!(["a", "b", "c"])));
  assert!(!or_type.is_assignable(&json!([1, "a", true]))); // Mezcla no permitida.
}

#[test]
fn custom() {
  let custom_type = Type::Custom("a".into());
  assert!(!custom_type.is_assignable(&json!("a")));
}