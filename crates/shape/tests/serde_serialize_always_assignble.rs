#![allow(unused)]

mod common;

use serde::Serialize;
use shape::{IsAsignable, Shape};

// A struct with named fields

#[derive(Serialize, Shape, Clone)]
struct NamedStruct {
  name: String,
  age: u8,
  active: bool,
}

// A struct with unnamed fields (tuple struct)

#[derive(Serialize, Shape, Clone)]
struct TupleStruct(String, u8, bool);

// A unit struct (no fields)

#[derive(Serialize, Shape, Clone)]
struct UnitStruct;

// Enum with different variants

#[derive(Serialize, Shape, Clone)]
enum ExampleEnum {
  // Unit variant
  Unit,
  // Tuple variant
  Tuple(u8, String),
  // Struct-like variant
  Struct { id: u32, label: String },
}

// Serialize a tuple

#[derive(Serialize, Shape, Clone)]
struct TupleExample(
  i32,
  f64,
  (String, bool), // Nested tuple
);

// Serialize primitives

#[derive(Serialize, Shape, Clone)]
struct PrimitiveExample {
  boolean: bool,
  integer: i64,
  float: f64,
  string: String,
  option: Option<String>,
  vec: Vec<u8>,
  hashmap: std::collections::HashMap<String, i32>,
}

// A nested struct

#[derive(Serialize, Shape, Clone)]
struct NestedStruct {
  inner: NamedStruct,
  examples: Vec<ExampleEnum>,
}

#[derive(Serialize, Shape, Clone)]
#[serde(rename_all = "camelCase")]
struct RenamedFields {
  user_id: String,
  first_name: String,
  last_name: String,
}

#[derive(Serialize, Shape, Clone)]
#[serde(rename_all = "camelCase")]
struct UserData {
  #[serde(rename = "id")]
  user_id: String,
  #[serde(skip)]
  internal_data: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  optional_field: Option<String>,
}

#[derive(Serialize, Shape, Clone)]
#[serde(rename_all = "snake_case", default)]
struct ConfigSettings {
  #[serde(default = "default_port")]
  port: u16,
  #[serde(skip_deserializing)]
  computed_value: String,
  enabled: bool,
}

#[derive(Serialize, Shape, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", rename_all_fields = "camelCase")]
enum Status {
  ActiveUser {
    user_name: String,
    login_count: u32,
  },
  PendingApproval {
    request_date: String,
  },
  #[serde(rename = "BLOCKED")]
  Suspended {
    reason_code: u32,
  },
}

#[derive(Serialize, Shape, Clone)]
#[serde(rename_all = "kebab-case")]
struct Metadata {
  #[serde(skip_serializing_if = "Vec::is_empty")]
  tags: Vec<String>,
  #[serde(default)]
  priority: i32,
  #[serde(rename = "createTime")]
  created_at: String,
}

fn serialize_is_assignable<T: Serialize + Shape>(v: T) {
  let ty = T::shape(&shape::ShapeOptions::for_serialize());
  assert!(ty.is_assignable(&serde_json::to_value(v).unwrap()));
}

#[test]
fn test_renamed_fields() {
  let data = RenamedFields {
    user_id: "123".to_string(),
    first_name: "John".to_string(),
    last_name: "Doe".to_string(),
  };
  serialize_is_assignable(&data);
}

#[test]
fn test_user_data() {
  let data = UserData {
    user_id: "456".to_string(),
    internal_data: "secret".to_string(),
    optional_field: Some("exists".to_string()),
  };
  serialize_is_assignable(&data);
}

#[test]
fn test_config_settings() {
  let data = ConfigSettings {
    port: 8080,
    computed_value: "computed".to_string(),
    enabled: true,
  };
  serialize_is_assignable(&data);
}

#[test]
fn test_status() {
  let data = Status::ActiveUser {
    user_name: "alice".to_string(),
    login_count: 5,
  };
  serialize_is_assignable(&data);
}

#[test]
fn test_metadata() {
  let data = Metadata {
    tags: vec!["test".to_string(), "prod".to_string()],
    priority: 1,
    created_at: "2023-01-01".to_string(),
  };
  serialize_is_assignable(&data);
}

#[test]
fn test_named_struct() {
  let data = NamedStruct {
    name: "Alice".to_string(),
    age: 30,
    active: true,
  };
  serialize_is_assignable(&data);
}

#[test]
fn test_tuple_struct() {
  let data = TupleStruct("Bob".to_string(), 25, false);
  serialize_is_assignable(&data);
}

#[test]
fn test_unit_struct() {
  let data = UnitStruct;
  serialize_is_assignable(&data);
}

#[test]
fn test_tuple_example() {
  let data = TupleExample(42, 3.4, ("Hello".to_string(), true));
  serialize_is_assignable(&data);
}

#[test]
fn test_primitive_example() {
  let mut hashmap = std::collections::HashMap::new();
  hashmap.insert("key1".to_string(), 100);

  let data = PrimitiveExample {
    boolean: true,
    integer: -42,
    float: 2.7,
    string: "Test".to_string(),
    option: Some("Optional".to_string()),
    vec: vec![1, 2, 3],
    hashmap,
  };
  let json = serde_json::to_string(&data).unwrap();
  serialize_is_assignable(&data);
}

#[test]
fn test_nested_struct() {
  let inner = NamedStruct {
    name: "Inner".to_string(),
    age: 20,
    active: true,
  };
  let examples = vec![
    ExampleEnum::Unit,
    ExampleEnum::Tuple(1, "Test".to_string()),
  ];

  let data = NestedStruct { inner, examples };
  serialize_is_assignable(&data);
}
