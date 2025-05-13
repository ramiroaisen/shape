pub use shape_macros::Shape;

mod to_typescript;
mod is_assignable;

pub use to_typescript::ToTypescript;
pub use indexmap;
pub use is_assignable::IsAsignable;

use indexmap::IndexMap;
use std::{
  borrow::Cow, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, rc::Rc, sync::Arc
};

/// The shape trait is derived in a type to generate a schema for the (de)serialization of that type
pub trait Shape {
  fn shape(options: &ShapeOptions) -> Type;
}

#[derive(Debug, Clone, Copy)]
pub enum ShapeOptionsKind {
  Serialize,
  Deserialize,
}

#[derive(Debug, Clone)]
pub struct ShapeOptions {
  pub kind: ShapeOptionsKind,
  pub option_is_optional: bool,
  pub option_add_undefined: bool,
  pub option_add_null: bool,
}

impl ShapeOptions {
  pub fn for_serialize() -> Self {
    Self {
      kind: ShapeOptionsKind::Serialize,
      option_is_optional: false,
      option_add_undefined: false,
      option_add_null: true,
    }
  }

  pub fn for_deserialize() -> Self {
    Self {
      kind: ShapeOptionsKind::Deserialize,
      option_is_optional: true,
      option_add_undefined: true,
      option_add_null: true,
    }
  }

  pub fn is_serialize(&self) -> bool {
    matches!(self.kind, ShapeOptionsKind::Serialize)
  }

  pub fn is_deserialize(&self) -> bool {
    matches!(self.kind, ShapeOptionsKind::Deserialize)
  }
}

/// This type tries to match the way JSON serialized Rust structs can be represented in typescript
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  Null,
  Undefined,
  Never,
  Boolean,
  Number,
  String,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
  pub optional: bool,
  pub readonly: bool,
  pub key: Box<Type>,
  pub value: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
  pub optional: bool,
  pub readonly: bool,
  pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Boolean(bool),
  Number(f64),
  String(String),
}

macro_rules! impl_ty {
  ($ty:ty, $value:expr) => {
    impl Shape for $ty {
      fn shape(_: &ShapeOptions) -> Type {
        $value
      }
    }
  };
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

impl_ty!(std::net::IpAddr, Type::String);
impl_ty!(std::net::Ipv4Addr, Type::String);
impl_ty!(std::net::Ipv6Addr, Type::String);

impl<T: Shape + ToOwned> Shape for Cow<'_, T> {
  fn shape(options: &ShapeOptions) -> Type {
    T::shape(options)
  }
}


#[cfg(feature = "time-0_3")]
impl_ty!(time::PrimitiveDateTime, Type::String);
#[cfg(feature = "time-0_3")]
impl_ty!(time::OffsetDateTime, Type::String);
#[cfg(feature = "compact-str-0_8")]
impl_ty!(compact_str::CompactString, Type::String);
#[cfg(feature = "bson")]
impl_ty!(bson::DateTime, Type::Custom("Date".to_string()));

#[cfg(feature = "ordered-float")]
impl<T> Shape for ordered_float::OrderedFloat<T> {
  fn shape(_: &ShapeOptions) -> Type {
    Type::Number
  }
}


impl<T: Shape + ?Sized> Shape for &T {
  fn shape(options: &ShapeOptions) -> Type {
    T::shape(options)
  }
}

impl<T: Shape> Shape for Option<T> {
  fn shape(options: &ShapeOptions) -> Type {
    let inner = T::shape(options);
    if options.option_add_null && options.option_add_undefined {
      Type::Or(vec![inner, Type::Null, Type::Undefined])
    } else if options.option_add_null {
      Type::Or(vec![inner, Type::Null])
    } else if options.option_add_undefined {
      Type::Or(vec![inner, Type::Undefined])
    } else {
      inner
    }
  }
}

macro_rules! impl_inner {
  ($ty:ty, $inner:ident) => {
    impl<$inner> Shape for $ty
    where
      $inner: Shape + ?Sized,
    {
      fn shape(options: &ShapeOptions) -> Type {
        <$inner>::shape(options)
      }
    }
  };
}

// TODO: add generics for Alloc in nightly
impl_inner!(Box<T>, T);
impl_inner!(Rc<T>, T);
impl_inner!(Arc<T>, T);


macro_rules! impl_slice {
  ($inner:ty, $($tt:tt)*) => {
    $($tt)*
    {
      fn shape(options: &ShapeOptions) -> Type {
        Type::Array(Array {
          item: Box::new(<$inner>::shape(options)),
        })
      }
    }
  };
}

// TODO: add generics for Alloc in nightly
impl_slice!(T, impl<T: Shape> Shape for [T]);
impl_slice!(T, impl<T: Shape> Shape for Vec<T>);
impl_slice!(T, impl<T: Shape, H> Shape for HashSet<T, H>);
impl_slice!(T, impl<T: Shape> Shape for BTreeSet<T>);
#[cfg(feature = "indexmap")]
impl_slice!(T, impl<T: Shape> Shape for indexmap::IndexSet<T>);

macro_rules! impl_map {
  ($k:ty, $v:ty, $($tt:tt)*) => {
    $($tt)*
    {
      fn shape(options: &ShapeOptions) -> Type {
        Type::Record(Record {
          optional: true,
          readonly: false,
          key: Box::new(<$k>::shape(options)),
          value: Box::new(<$v>::shape(options)),
        })
      }
    }
  };
}

// TODO: add generics for Alloc in nightly
impl_map!(K, V, impl<K: Shape, V: Shape, H> Shape for HashMap<K, V, H>);
impl_map!(K, V, impl<K: Shape, V: Shape> Shape for BTreeMap<K, V>);
#[cfg(feature = "intmap")]
impl_map!(K, V, impl<K: Shape, V: Shape> Shape for intmap::IntMap<K, V>);
#[cfg(feature = "inttable")]
impl_map!(u64, V, impl<V: Shape> Shape for inttable::IntTable<V>);
#[cfg(feature = "indexmap")]
impl_map!(K, V, impl<K: Shape, V: Shape, H> Shape for indexmap::IndexMap<K, V, H>);


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

impl<T, const N: usize> Shape for [T; N]
where
  T: Shape,
{
  fn shape(options: &ShapeOptions) -> Type {
    let inner = T::shape(options);
    let mut items = Vec::with_capacity(N);
    for _ in 0..N {
      items.push(inner.clone());
    }
    Type::Tuple(Tuple { items, rest: None })
  }
}

// #[doc(hidden)]
// pub mod internal {
//     use std::any::TypeId;

//   pub trait IsOption {
//     fn is_option<I: 'static>() -> bool;
//   }

//   impl<T: 'static> IsOption for T {
//     fn is_option<I: 'static>() -> bool {
//       TypeId::of::<T>() == TypeId::of::<Option<I>>()
//     }
//   }
// }
