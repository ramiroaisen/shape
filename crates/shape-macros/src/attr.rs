use darling::{FromAttributes, FromMeta};
use syn::Type;

#[derive(Debug, FromAttributes)]
#[darling(attributes(serde, shape))]
pub struct ContainerAttrs {
  pub rename_all: Option<Complex<Inflection>>,
  pub rename_all_fields: Option<Complex<Inflection>>,
  pub tag: Option<String>,
  pub content: Option<String>,
  pub untagged: Option<()>,
  pub transparent: Option<()>,
  pub from: Option<Type>,
  pub try_from: Option<Type>,
  pub into: Option<Type>,
  pub try_into: Option<Type>,
  pub default: Option<UnitOr<String>>,
  
  // unused
  // pub rename: Option<Complex<String>>,
  // pub deny_unknown_fields: Option<()>,
  // pub remote: Option<Type>,
  // #[darling(rename = "crate")]
  // pub serde_crate: Option<String>, 
  // pub expecting: Option<String>,
  // pub bound: Option<Complex<String>>,
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(serde, shape))]
pub struct VariantAttrs {
  pub rename: Option<Complex<String>>,
  pub rename_all: Option<Complex<Inflection>>,
  pub skip: Option<()>,
  pub skip_serializing: Option<()>,
  pub skip_deserializing: Option<()>,
  pub untagged: Option<()>,

  // unused
  // pub alias: Option<String>,
  // pub with: Option<String>,
  // pub serialize_with: Option<String>,
  // pub deserialize_with: Option<String>,
  // pub bound: Option<Complex<String>>,
  // pub borrow: Option<UnitOr<String>>,
  // pub other: Option<()>,
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(serde, shape))]
pub struct FieldAttrs {
  pub rename: Option<Complex<String>>,
  pub default: Option<UnitOr<String>>,
  pub flatten: Option<()>,
  pub skip: Option<()>,
  pub skip_serializing: Option<()>,
  pub skip_deserializing: Option<()>,
  pub skip_serializing_if: Option<String>,
  
  // unused
  // pub alias: Option<String>,
  // pub with: Option<String>,
  // pub serialize_with: Option<String>,
  // pub deserialize_with: Option<String>,
  // pub borrow: Option<UnitOr<String>>, 
  // pub bound: Option<Complex<String>>,
  // pub getter: Option<String>,
}


#[derive(Debug, FromMeta)]
pub enum Inflection {
  #[darling(rename = "lowercase")]
  Lower,
  #[darling(rename = "UPPERCASE")]
  Upper,
  #[darling(rename = "PascalCase")]
  Pascal,
  #[darling(rename = "camelCase")]
  Camel,
  #[darling(rename = "snake_case")]
  Snake,
  #[darling(rename = "SCREAMING_SNAKE_CASE")]
  ScreamingSnake,
  #[darling(rename = "kebab-case")]
  Kebab,
  #[darling(rename = "SCREAMING-KEBAB-CASE")]
  ScreamingKebab,
}

impl Inflection {
  pub fn apply(&self, value: &str) -> String {
    use inflector::Inflector;
    match self {
      Inflection::Lower => value.to_lowercase(),
      Inflection::Upper => value.to_uppercase(),
      Inflection::Pascal => value.to_pascal_case(), 
      Inflection::Camel => value.to_camel_case(),
      Inflection::Snake => value.to_snake_case(),
      Inflection::ScreamingSnake => value.to_screaming_snake_case(),
      Inflection::Kebab => value.to_kebab_case(),
      Inflection::ScreamingKebab => value.to_kebab_case().to_uppercase(),
    }
  }
}


#[derive(Debug, FromMeta)]
pub struct SerDe<T> {
  pub serialize: Option<T>,
  pub deserialize: Option<T>,
}

#[derive(Debug)]
pub enum Complex<T> {
  Single(T),
  Complex {
    serialize: Option<T>,
    deserialize: Option<T>,
  }
}

impl<T> From<SerDe<T>> for Complex<T> {
  fn from(value: SerDe<T>) -> Self {
    Self::Complex {
      serialize: value.serialize,
      deserialize: value.deserialize,
    }
  }
}

impl<T: FromMeta> FromMeta for Complex<T> {
  
  fn from_nested_meta(src: &darling::ast::NestedMeta) -> Result<Self, darling::Error> {
    match SerDe::<T>::from_nested_meta(src) {
      Ok(complex) => {
        Ok(complex.into())
      },
      Err(_) => {
        Ok(Complex::Single(T::from_nested_meta(src)?))
      } 
    }
  }

  fn from_meta(src: &syn::Meta) -> Result<Self, darling::Error> {
    match SerDe::<T>::from_meta(src) {
      Ok(complex) => {
        Ok(complex.into())
      },
      Err(_) => {
        Ok(Complex::Single(T::from_meta(src)?))
      }
    }
  }

  fn from_none() -> Option<Self> {
    match SerDe::<T>::from_none() {
      Some(complex) => {
        Some(complex.into())
      },
      None => {
        Some(Complex::Single(T::from_none()?))
      }
    }
  }

  fn from_word() -> Result<Self, darling::Error> {
    match SerDe::<T>::from_word() {
      Ok(complex) => {
        Ok(complex.into())
      },
      Err(_) => {
        Ok(Complex::Single(T::from_word()?))
      }
    }
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> Result<Self, darling::Error> {
    match SerDe::<T>::from_list(items) {
      Ok(complex) => {
        Ok(complex.into())
      },
      Err(_) => {
        Ok(Complex::Single(T::from_list(items)?))
      }
    }
  }
  
  fn from_value(value: &syn::Lit) -> Result<Self, darling::Error> {
    match SerDe::<T>::from_value(value) {
      Ok(complex) => {
        Ok(complex.into())
      },
      Err(_) => {
        Ok(Complex::Single(T::from_value(value)?))
      }
    }
  }

  fn from_expr(expr: &syn::Expr) -> Result<Self, darling::Error> {
    match SerDe::<T>::from_expr(expr) {
      Ok(complex) => {
        Ok(complex.into())
      },
      Err(_) => {
        Ok(Complex::Single(T::from_expr(expr)?))
      }
    }
  }

  fn from_char(value: char) -> Result<Self, darling::Error> {
    match SerDe::<T>::from_char(value) {
      Ok(complex) => {
        Ok(complex.into())
      },
      Err(_) => {
        Ok(Complex::Single(T::from_char(value)?))
      }
    }
  }

  fn from_string(value: &str) -> Result<Self, darling::Error> {
    match SerDe::<T>::from_string(value) {
      Ok(complex) => {
        Ok(complex.into())
      },
      Err(_) => {
        Ok(Complex::Single(T::from_string(value)?))
      }
    }
  }

  fn from_bool(value: bool) -> Result<Self, darling::Error> {
    match SerDe::<T>::from_bool(value) {
      Ok(complex) => {
        Ok(complex.into())
      },
      Err(_) => {
        Ok(Complex::Single(T::from_bool(value)?))
      }
    }
  }
}


#[derive(Debug)]
pub enum UnitOr<T> {
  Unit,
  Value(T),
}

impl<T: FromMeta> FromMeta for UnitOr<T> {
  fn from_nested_meta(src: &darling::ast::NestedMeta) -> Result<Self, darling::Error> {
    match T::from_nested_meta(src) {
      Ok(v) => Ok(Self::Value(v)),
      Err(_) => {
        <()>::from_nested_meta(src)?;
        Ok(Self::Unit)
      }
    }
  }

  fn from_bool(value: bool) -> darling::Result<Self> {
    match T::from_bool(value) {
      Ok(v) => Ok(Self::Value(v)),
      Err(_) => {
        <()>::from_bool(value)?;
        Ok(Self::Unit)
      }
    }
  }

  fn from_char(value: char) -> Result<Self, darling::Error> {
    match T::from_char(value) {
      Ok(v) => Ok(Self::Value(v)),
      Err(_) => {
        <()>::from_char(value)?;
        Ok(Self::Unit)
      }
    }
  }

  fn from_string(value: &str) -> Result<Self, darling::Error> {
    match T::from_string(value) {
      Ok(v) => Ok(Self::Value(v)),
      Err(_) => {
        <()>::from_string(value)?;
        Ok(Self::Unit)
      }
    }
  }

  fn from_expr(expr: &syn::Expr) -> Result<Self, darling::Error> {
    match T::from_expr(expr) {
      Ok(v) => Ok(Self::Value(v)),
      Err(_) => {
        <()>::from_expr(expr)?;
        Ok(Self::Unit)
      }
    }
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> Result<Self, darling::Error> {
    match T::from_list(items) {
      Ok(v) => Ok(Self::Value(v)),
      Err(_) => {
        <()>::from_list(items)?;
        Ok(Self::Unit)
      }
    }
  }

  fn from_meta(item: &syn::Meta) -> Result<Self, darling::Error> {
    match T::from_meta(item) {
      Ok(v) => Ok(Self::Value(v)),
      Err(_) => {
        <()>::from_meta(item)?;
        Ok(Self::Unit)
      }
    }
  }

  fn from_value(value: &syn::Lit) -> Result<Self, darling::Error> {
    match T::from_value(value) {
      Ok(v) => Ok(Self::Value(v)),
      Err(_) => {
        <()>::from_value(value)?;
        Ok(Self::Unit)
      }
    }
  }

  fn from_word() -> Result<Self, darling::Error> {
    match T::from_word() {
      Ok(v) => Ok(Self::Value(v)),
      Err(_) => {
        <()>::from_word()?;
        Ok(Self::Unit)
      }
    }
  }

  fn from_none() -> Option<Self> {
    match T::from_none() {
      Some(v) => Some(Self::Value(v)),
      None => <()>::from_none().map(|_| Self::Unit)
    }
  }
}