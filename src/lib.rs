//! Do you need trait objects, in the form `dyn MyTrait`, that implement traits which are not object-safe?
//!
//! This crate solves the problem by providing object-safe traits which are analogous to some commonly used traits that are not object-safe, and auto-implementing both for a wide range of types. Currently, the following traits are supported:
//! - Hash
//! - PartialEq
//! - Eq
//!
//! I plan to extend this support to other traits, and offer macros to simplify the process for custom traits.
//!
//! Learn about object safety here: https://doc.rust-lang.org/reference/items/traits.html#object-safety
//!
//! ## Example
//!
//! Let's take the `Hash` trait as an example. `Hash` is not object-safe. This means that `dyn Hash` is not a valid type in rust. Now, imagine you define this custom trait:
//! ```rust
//! pub trait MyTrait: Hash {}
//! ```
//! Since `MyTrait` extends `Hash`, it is not object safe either, and `dyn MyTrait` is not a valid type. This crate offers a way to work around this limitation, so you can have object-safe traits whose objects implement non-object-safe traits such as `Hash`.
//!
//! Instead of expressing `Hash` as the trait bound, express `HashObject` as the trait bound.
//! ```rust
//! pub trait MyTrait: HashObject {}
//! ```
//!
//! You do not need to implement `HashObject`. It is automatically implemented for any type that implements `Hash`. Now, `dyn MyTrait` is object-safe. Add one line of code if you want `dyn MyTrait` to implement `Hash`:
//!
//! ```rust
//! impl_hash(dyn MyTrait);
//! ```
//!
//! Here are all the characteristics that come with HashObject:
//! - anything implementing `Hash` automatically implements `HashObject`
//! - `dyn HashObject` implements `Hash`.
//! - `Object<T>` implements `Hash` for any `T: HashObject`. The `T` can be `Box<dyn MyTrait>`, for example.
//! - `impl_hash` can implement `Hash` for any type that implements `HashObject`, for example a trait object `dyn MyTrait` where `MyTrait` is a trait extending `HashObject`.
//!
//! ```rust
//! impl_hash! {
//!     // typical use, where MyTrait: HashObject
//!     dyn MyTrait,
//!     dyn AnotherTrait,
//!
//!     // structs and enums are supported if they deref to a target that implements HashObject or Hash.
//!     MyStruct,
//!
//!     // special syntax for generics.
//!     MySimpleGeneric<T> where <T>,
//!     MyGenericType<T, F> where <T, F: HashObject>,
//!     dyn MyGenericTrait<T> where <T: SomeTraitBound>,
//! }
//! ```

use core::{
    any::Any,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, Debug)]
pub struct Object<T>(pub T);

impl<T> Deref for Object<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Object<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Helper trait to enable trait upcasting, since upcasting is not stable.
pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &(dyn Any) {
        self as &dyn Any
    }
}

/// Object-safe version of Eq
pub trait EqObject: PartialEqObject {
    fn as_eq_object(&self) -> &dyn EqObject;
}

impl<T> EqObject for T
where
    T: Eq + PartialEqObject,
{
    fn as_eq_object(&self) -> &dyn EqObject {
        self
    }
}

impl_eq! {
    Object<T> where <T: EqObject>,
    dyn EqObject,
}

#[macro_export]
macro_rules! impl_eq {
    ($( $Type:ty $(where <$($G:ident: $Gb:ident),+>)? ),*$(,)?) => {
        $(impl$(<$($G: $Gb),+>)? Eq for $Type {})*
    };
}

/// Object-safe version of PartialEq
pub trait PartialEqObject: AsAny {
    fn eq_object(&self, other: &dyn PartialEqObject) -> bool;
    fn as_partial_eq_object(&self) -> &dyn PartialEqObject;
}

impl<T> PartialEqObject for T
where
    T: PartialEq + AsAny,
{
    fn eq_object(&self, other: &dyn PartialEqObject) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            Some(other) => self == other,
            None => false,
        }
    }

    fn as_partial_eq_object(&self) -> &dyn PartialEqObject {
        self
    }
}

impl_partial_eq! {
    Object<T> where <T: PartialEqObject>,
    dyn PartialEqObject,
    dyn EqObject,
}

#[macro_export]
macro_rules! impl_partial_eq {
    ($( $Type:ty $(where <$($G:ident: $Gb:ident),+>)? ),*$(,)?) => {
        $(impl$(<$($G: $Gb),+>)? PartialEq for $Type {
            fn eq(&self, other: &Self) -> bool {
                self.eq_object(other.as_partial_eq_object())
            }
        })*
    };
}

/// Object-safe version of `std::hash::Hash`
pub trait HashObject {
    fn hash_object(&self, state: &mut dyn Hasher);
    fn as_hash_object(&self) -> &dyn HashObject;
}

impl<T: Hash> HashObject for T {
    fn hash_object(&self, mut state: &mut dyn Hasher) {
        self.hash(&mut state);
    }

    fn as_hash_object(&self) -> &dyn HashObject {
        self
    }
}

impl_hash! {
    Object<T> where <T: HashObject>,
    dyn HashObject,
}

#[macro_export]
macro_rules! impl_hash {
    ($( $Type:ty $(where <$($G:ident: $Gb:ident),+>)? ),*$(,)?) => {
        $(impl$(<$($G: $Gb),+>)? std::hash::Hash for $Type {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.hash_object(state);
            }
        })*
    };
}

#[cfg(test)]
mod test {
    use std::collections::hash_map::DefaultHasher;

    use super::*;

    #[test]
    fn eq() {
        let x: Box<dyn EqObject> = Box::new(10);
        let y: Box<dyn EqObject> = Box::new(10);
        let z: Box<dyn EqObject> = Box::new(11);
        if x != y {
            panic!("should be equal")
        }
        if x == z {
            panic!("should not be equal")
        }
    }

    #[test]
    fn hash_works() {
        let x: &str = "Hello, World!";
        let y: &dyn HashObject = "Hello, World!".as_hash_object();
        let z: &dyn HashObject = "banana".as_hash_object();
        assert_eq!(hash(x), hash(y));
        assert_ne!(hash(y), hash(z));
    }

    fn hash<T: Hash>(t: T) -> u64 {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        hasher.finish()
    }
}

// /// TODO:
// /// - handle different method signature between declaration and definition
// /// - create impl_* macro
// /// - better syntax, find a way around square brackets
// /// - converting this to a proc macro is probably best
// ///
// /// wip! {
// ///     PartialEq: AsAny {
// ///         [fn eq_object(&self, other: &dyn PartialEqObject) -> bool] {
// ///             match other.as_any().downcast_ref::<Self>() {
// ///                 Some(other) => self == other,
// ///                 None => false,
// ///             }
// ///         }
// ///     }
// ///
// ///     Eq: PartialEqObject {}
// /// }
// #[allow(unused)]
// macro_rules! wip {
//     (
//         $(
//             $Trait:ty $(: $($TraitBound:ty)+)? $(where T: $($ImplBound:ty)+)?
//             {$(
//                 [$($fn_sig:tt)*]
//                 $fn_impl:block
//             )*}
//         )*
//     ) => {$(paste::paste!{
//         pub trait [<$Trait Object>] $(: $($TraitBound)++)? {
//             fn [<as_ $Trait:snake _object>](&self) -> &dyn [<$Trait Object>];

//             $($($fn_sig)*;)*
//         }

//         impl<T> [<$Trait Object>] for T
//         where
//             T: $Trait $($(+ $TraitBound)+)?,
//         {
//             fn [<as_ $Trait:snake _object>](&self) -> &dyn [<$Trait Object>] {
//                 self
//             }

//             $($($fn_sig)* {$fn_impl})*
//         }
//     })*};
// }
