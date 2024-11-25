use crate::{Array, Literal, Object, Record, Tuple, Type};

pub trait ToTypescript {
  fn to_typescript(&self) -> String;
}

impl ToTypescript for Array {
  fn to_typescript(&self) -> String {
    format!("Array<{}>", self.item.to_typescript())
  }
}

impl ToTypescript for Object {
  fn to_typescript(&self) -> String {
    let mut properties = vec![];
    for (key, prop) in self.properties.iter() {
      
      macro_rules! quote {
        ($key:expr) => {
          serde_json::to_string($key).unwrap()
        };
      }

      let quoted_key = {
        let first = key.chars().nth(0);
        match first {
          None => String::from("\"\""),
          Some(first) => {
            if
              !matches!(first, 'a'..='z' | 'A'..='Z' | '_') ||
              key.contains(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')) 
            {
               quote!(key)
            } else {
              String::from(key)
            }
          }
        }
      };

      properties.push(
        format!(
          "{readonly}{key}{optional}: {value};",
          readonly = if prop.readonly { "readonly " } else { "" },
          key = quoted_key,
          optional = if prop.optional { "?" } else { "" },
          value = prop.ty.to_typescript(),
        )
      );
    }
    format!("{{ {} }}", properties.join(" "))
  }
}

impl ToTypescript for Record {
  fn to_typescript(&self) -> String {
    format!(
      "{{ [key: {key}]: {value} }}",
      key = self.key.to_typescript(),
      value = self.value.to_typescript()
    )
  }
}

impl ToTypescript for Literal {
  fn to_typescript(&self) -> String {
      match self {
        Literal::String(value) => serde_json::to_string(value).unwrap(),
        Literal::Number(value) => value.to_string(),
        Literal::Boolean(value) => value.to_string(),
      }
  }
}

impl ToTypescript for Tuple {
  fn to_typescript(&self) -> String {
    let inner = self.items.iter().map(|t| t.to_typescript()).collect::<Vec<String>>().join(", ");
    format!("[{}]", inner)
  }
}

impl ToTypescript for Type {
  fn to_typescript(&self) -> String {
    match self {
      Type::String => String::from("string"),
      Type::Number => String::from("number"),
      Type::Boolean => String::from("boolean"),
      Type::Null => String::from("null"),
      Type::Undefined => String::from("undefined"),
      Type::Never => String::from("never"),
      Type::Literal(literal) => literal.to_typescript(),
      Type::Tuple(tuple) => tuple.to_typescript(),
      Type::Array(array) => array.to_typescript(),
      Type::Object(object) => object.to_typescript(),
      Type::Record(record) => record.to_typescript(),
      Type::And(types) => {
        if types.is_empty() {
          return String::from("never");  
        }
        let inner = types.iter().map(|t| t.to_typescript()).collect::<Vec<String>>().join(" & ");
        format!("({})", inner)
      }
      Type::Or(types) => {
        if types.is_empty() {
          return String::from("never");  
        }
        let inner = types.iter().map(|t| t.to_typescript()).collect::<Vec<String>>().join(" | ");
        format!("({})", inner)
      }
      Type::Custom(custom) => custom.clone(),
    }
  }
}