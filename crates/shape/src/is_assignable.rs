use serde_json::{json, Map, Value};

use crate::{Array, Literal, Object, Record, Tuple, Type};

pub trait IsAsignable {
  fn is_assignable(&self, v: &Value) -> bool;  
}

impl IsAsignable for Literal {
  fn is_assignable(&self, v: &Value) -> bool {
      match (self, v) {
        (Self::String(l), Value::String(v)) => l == v,
        (Self::Number(l), Value::Number(v)) => match v.as_f64() {
          Some(v) => *l == v,
          None => false,
        },
        (Self::Boolean(l), Value::Bool(v)) => l == v,
        _ => false,
      }
  }
}

impl IsAsignable for Tuple {
  fn is_assignable(&self, v: &Value) -> bool {
    match v {
      Value::Array(items) => {
        for (i, t) in self.items.iter().enumerate() {
          if !t.is_assignable(&items[i]) {
            return false;
          }
        }
        
        match &self.rest {
          None => true,
          Some(rest) => {
            items.iter().skip(self.items.len()).all(|item| rest.is_assignable(item))  
          }
        }

      }
      _ => false,
    }
  }
}

impl IsAsignable for Array {
  fn is_assignable(&self, v: &Value) -> bool {
    match v {
      Value::Array(items) => {
        items.iter().all(|item| self.item.is_assignable(item))
      }
      _ => false,
    }
  }
}

impl IsAsignable for Object {
  fn is_assignable(&self, v: &Value) -> bool {
    match v {
      Value::Object(map) => {
        self.properties.iter().all(|(key, prop)| {
          let v = map.get(key);
          match v {
            None => prop.optional,
            Some(v) => prop.ty.is_assignable(v),
          }
        })
      },
      _ => false,
    }
  }
}

impl IsAsignable for Record {
  fn is_assignable(&self, v: &Value) -> bool {
    match v {
      Value::Object(map) => {
        // A partial struct is assignable to all structs
        if self.optional {
          return true
        }

        fn match_key(key: &Type, ty: &Type, map: &Map<String, Value>) -> bool { 
          match key {
            Type::Null => false,
            Type::Undefined => false,
            Type::Boolean => false,
            Type::Tuple(_) => false,
            Type::Array(_) => false,
            Type::Object(_) => false,
            Type::Record(_) => false,
            Type::Custom(_) => false,
            Type::Never => false,
            Type::String => {
              map.iter().all(|(_, v)| {
                ty.is_assignable(v)
              })
            },
            Type::Number => {
              map.iter().all(|(k, v)| {
                use std::str::FromStr;
                match f64::from_str(k) {
                  Err(_) => false,
                  Ok(_) => ty.is_assignable(v)
                }
              })
            },
            Type::And(iter) => {
              map.iter().all(|(k, v)| {
                if iter.iter().all(|k_each| k_each.is_assignable(&json!(k))) {
                  ty.is_assignable(v)
                } else {
                  true
                }
              })
            },
            Type::Or(iter) => iter.iter().all(|k| match_key(k, ty, map)),
            Type::Literal(lit) => match lit {
              Literal::Boolean(_) => false,
              Literal::String(s) => {
                map.iter().any(|(k, v)| {
                  s == k && ty.is_assignable(v)
                })
              }
              Literal::Number(n) => {
                use std::str::FromStr;
                map.iter().any(|(k, v)| {
                  f64::from_str(k) == Ok(*n) && ty.is_assignable(v)
                })
              }
            }
          }
        }

        match_key(&self.key, &self.value, map)        
      },
      _ => false,
    }
  }
}

impl IsAsignable for Type {
  fn is_assignable(&self, v: &Value) -> bool {
    match self {
      Type::String => v.is_string(),
      Type::Number => v.is_number(),
      Type::Boolean => v.is_boolean(),
      Type::Null => v.is_null(),
      Type::Undefined => false,
      Type::Never => false,
      Type::Literal(literal) => literal.is_assignable(v),
      Type::Tuple(tuple) => tuple.is_assignable(v),
      Type::Array(array) => array.is_assignable(v),
      Type::Object(object) => object.is_assignable(v),
      Type::Record(record) => record.is_assignable(v),
      Type::And(types) => types.iter().all(|t| t.is_assignable(v)),
      Type::Or(types) => types.iter().any(|t| t.is_assignable(v)),
      Type::Custom(_) => false,
    }
  }
}