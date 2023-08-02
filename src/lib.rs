//! With this crate, you can create trait objects that implement traits which
//! are not object safe. This is done by providing an analogous object-safe
//! trait to the trait that is not object-safe, and auto-implementing both for a
//! wide range of types.
//!
//! Let's say there is some trait `Hash` that is not object safe, but you want a
//! trait object that implements `Hash`.  This crate provides a trait called
//! `HashObject`, which *is* object-safe, with the following characteristics:
//! - `HashObject` is auto-implemented for any type that implements `Hash`.
//! - `dyn HashObject` implements `Hash`.
//! - `Object<dyn T>` implements `Hash` for any trait T that extends
//!   `HashObject`.
//! - Use the `impl_hash` macro to easily implement `Hash` for any type that
//!   implements HashObject, for example a trait object `dyn T` where T is a
//!   trait extending `HashObject`.

use core::{
    any::Any,
    hash::{Hash, Hasher},
    ops::Deref,
};

pub struct Object<T>(pub T);

impl<T> Deref for Object<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
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
        $(impl$(<$($G: $Gb),+>)? Hash for $Type {
            fn hash<H: Hasher>(&self, state: &mut H) {
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
