use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet}, rc::Rc, sync::Arc};

pub use indexmap;
use indexmap::{IndexMap, IndexSet};

/// The shape trait is derived in a type to generate a schema for the (de)serialization of that type
pub trait Shape {
  fn shape(options: &ShapeOptions) -> Type;
}

#[derive(Debug, Clone)]
pub enum ShapeOptions {
  Serialize,
  Deserialize,  
}

impl ShapeOptions {
  pub fn is_serialize(&self) -> bool {
    matches!(self, ShapeOptions::Serialize { .. })
  }

  pub fn is_deserialize(&self) -> bool {
    matches!(self, ShapeOptions::Deserialize { .. })
  }
}

/// This type tries to match the way JSON serialized Rust structs can be represented in typescript
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  String,
  Number,
  Boolean,
  Null,
  Undefined,
  Never,
  Literal(Literal),
  Tuple(Tuple),
  Array(Array),
  Object(Object),
  Record(Record),
  And(Vec<Type>),
  Or(Vec<Type>),
  /// a way to declare a custom type Eg: #\[shape(type = "Date")\]
  Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tuple {
  pub items: Vec<Type>,
  pub rest: Option<Box<Type>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
  pub item: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
  pub properties: IndexMap<String, Property>,
}

impl Object {
  /// Make all properties in this Object optional
  pub fn partial(mut self) -> Self {
    for (_, prop) in self.properties.iter_mut() {
      prop.optional = true;
    }
    self
  }

  /// Make all properties in this Object non-optional
  pub fn required(mut self) -> Self {
    for (_, prop) in self.properties.iter_mut() {
      prop.optional = false;
    }
    self
  }

  /// Make all properties in this Object readonly
  pub fn readonly(mut self) -> Self {
    for (_, prop) in self.properties.iter_mut() {
      prop.readonly = true;
    }
    self
  }

  /// Make all properties in this Object non-readonly
  pub fn writable(mut self) -> Self {
    for (_, prop) in self.properties.iter_mut() {
      prop.readonly = false;
    }
    self
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
  pub key: Box<Type>,
  pub value: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
  pub ty: Type,
  pub optional: bool,
  pub readonly: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  String(String),
  Number(f64),
  Boolean(bool),
}

macro_rules! impl_ty {
  ($ty:ty, $value:expr) => {
    impl Shape for $ty {
      fn shape(_: &ShapeOptions) -> Type {
        $value
      }
    }
  }
}

impl_ty!(String, Type::String);
impl_ty!(str, Type::String);
impl_ty!(i8, Type::Number);
impl_ty!(i16, Type::Number);
impl_ty!(i32, Type::Number);
impl_ty!(i64, Type::Number);
impl_ty!(i128, Type::Number);
impl_ty!(isize, Type::Number);
impl_ty!(u8, Type::Number);
impl_ty!(u16, Type::Number);
impl_ty!(u32, Type::Number);
impl_ty!(u64, Type::Number);
impl_ty!(u128, Type::Number);
impl_ty!(usize, Type::Number);
impl_ty!(f32, Type::Number);
impl_ty!(f64, Type::Number);
impl_ty!(bool, Type::Boolean);
impl_ty!((), Type::Null);

impl<T: Shape + ?Sized> Shape for &T {
  fn shape(options: &ShapeOptions) -> Type {
    T::shape(options)
  }
} 


macro_rules! impl_inner {
  ($ty:ty, $inner:ident) => {
    impl<$inner> Shape for $ty where $inner: Shape {
      fn shape(options: &ShapeOptions) -> Type {
        <$inner>::shape(options)
      }
    }
  }
} 

impl_inner!(Box<T>, T);
impl_inner!(Rc<T>, T);
impl_inner!(Arc<T>, T);


macro_rules! impl_slice {
  ($ty:ty, $inner:ident) => {
    impl<$inner> Shape for $ty where $inner: Shape {
      fn shape(options: &ShapeOptions) -> Type {
        Type::Array(Array {
          item: Box::new(<$inner>::shape(options)),
        })
      }
    }
  } 
}

impl_slice!([T], T);
impl_slice!(Vec<T>, T);
impl_slice!(HashSet<T>, T);
impl_slice!(BTreeSet<T>, T);
impl_slice!(IndexSet<T>, T);

macro_rules! impl_map {
  ($ty:ty, $key:ident, $value:ident) => {
    impl<$key, $value> Shape for $ty where $key: Shape, $value: Shape {
      fn shape(options: &ShapeOptions) -> Type {
        Type::Record(Record {
          key: Box::new(<$key>::shape(options)),
          value: Box::new(<$value>::shape(options)),
        })
      }
    }
  }
}

impl_map!(HashMap<K, V>, K, V);
impl_map!(BTreeMap<K, V>, K, V);
impl_map!(IndexMap<K, V>, K, V);


macro_rules! impl_tuple {
  ($($ty:ident)*) => {
    impl<$($ty),*> Shape for ($($ty,)*) where $($ty: Shape),* {
      fn shape(options: &ShapeOptions) -> Type {
        Type::Tuple(Tuple {
          items: vec![
            $(<$ty>::shape(options)),*
          ],
          rest: None,
        })
      }
    }
  }
}

macro_rules! impl_tuple_all {
  ($first:ident) => {
    impl_tuple!($first);
  };

  ($first:ident $($rest:ident)*) => {
    impl_tuple!($first $($rest)*);
    impl_tuple_all!($($rest)*);
  }
}

impl_tuple_all!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29 T30 T31 T32);

impl<T, const N: usize> Shape for [T; N] where T: Shape {
  fn shape(options: &ShapeOptions) -> Type {
    let inner = T::shape(options);
    let mut items = Vec::with_capacity(N);
    for _ in 0..N {
      items.push(inner.clone());
    }
    Type::Tuple(Tuple {
      items,
      rest: None,
    })
  }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn primitives() {
    for options in &[ShapeOptions::Serialize, ShapeOptions::Deserialize] {
      assert_eq!(Type::String, String::shape(options));
      assert_eq!(Type::String, str::shape(options));
      assert_eq!(Type::Number, u8::shape(options));
      assert_eq!(Type::Number, u16::shape(options));
      assert_eq!(Type::Number, u32::shape(options));
      assert_eq!(Type::Number, u64::shape(options));
      assert_eq!(Type::Number, u128::shape(options));
      assert_eq!(Type::Number, usize::shape(options));
      assert_eq!(Type::Number, i8::shape(options));
      assert_eq!(Type::Number, i16::shape(options));
      assert_eq!(Type::Number, i32::shape(options));
      assert_eq!(Type::Number, i64::shape(options));
      assert_eq!(Type::Number, i128::shape(options));
      assert_eq!(Type::Number, isize::shape(options));
      assert_eq!(Type::Number, f32::shape(options));
      assert_eq!(Type::Number, f64::shape(options));
      assert_eq!(Type::Boolean, bool::shape(options));
      assert_eq!(Type::Null, <()>::shape(options));
    }
  }

  #[test]
  fn refs() {
    for options in &[ShapeOptions::Serialize, ShapeOptions::Deserialize] {
      assert_eq!(Type::String, <&String>::shape(options));
      assert_eq!(Type::String, <&str>::shape(options));
      assert_eq!(Type::Number, <&u8>::shape(options));
      assert_eq!(Type::Number, <&u16>::shape(options));
      assert_eq!(Type::Number, <&u32>::shape(options));
      assert_eq!(Type::Number, <&u64>::shape(options));
      assert_eq!(Type::Number, <&u128>::shape(options));
      assert_eq!(Type::Number, <&usize>::shape(options));
      assert_eq!(Type::Number, <&i8>::shape(options));
      assert_eq!(Type::Number, <&i16>::shape(options));
      assert_eq!(Type::Number, <&i32>::shape(options));
      assert_eq!(Type::Number, <&i64>::shape(options));
      assert_eq!(Type::Number, <&i128>::shape(options));
      assert_eq!(Type::Number, <&isize>::shape(options));
      assert_eq!(Type::Number, <&f32>::shape(options));
      assert_eq!(Type::Number, <&f64>::shape(options));
      assert_eq!(Type::Boolean, <&bool>::shape(options));
      assert_eq!(Type::Null, <&()>::shape(options));
    }
  }
}