# Object Safe

Do you need trait objects, in the form `dyn MyTrait`, that implement traits which are not object-safe?

This crate solves the problem by providing object-safe traits which are analogous to some commonly used traits that are not object-safe, and auto-implementing both for a wide range of types. Currently, the following traits are supported:
- Hash
- PartialEq
- Eq

I plan to extend this support to other traits, and offer macros to simplify the process for custom traits.

Learn about object safety here: https://doc.rust-lang.org/reference/items/traits.html#object-safety

## Example

Let's take the `Hash` trait as an example. `Hash` is not object-safe. This means that `dyn Hash` is not a valid type in rust. Now, imagine you define this custom trait:
```rust
pub trait MyTrait: Hash {}
```
Since `MyTrait` extends `Hash`, it is not object safe either, and `dyn MyTrait` is not a valid type. This crate offers a way to work around this limitation, so you can have object-safe traits whose objects implement non-object-safe traits such as `Hash`.

Instead of expressing `Hash` as the trait bound, express `HashObject` as the trait bound.
```rust
pub trait MyTrait: HashObject {}
```

You do not need to implement `HashObject`. It is automatically implemented for any type that implements `Hash`. Now, `dyn MyTrait` is object-safe. Add one line of code if you want `dyn MyTrait` to implement `Hash`:

```rust
impl_hash(dyn MyTrait);
```

Here are all the characteristics that come with HashObject:
- anything implementing `Hash` automatically implements `HashObject`
- `dyn HashObject` implements `Hash`.
- `Object<T>` implements `Hash` for any `T: HashObject`. The `T` can be `Box<dyn MyTrait>`, for example.
- `impl_hash` can implement `Hash` for any type that implements `HashObject`, for example a trait object `dyn MyTrait` where `MyTrait` is a trait extending `HashObject`.

```rust
impl_hash! {
    // typical use, where MyTrait: HashObject
    dyn MyTrait,
    dyn AnotherTrait,

    // structs and enums are supported if they deref to a target that implements HashObject or Hash.
    MyStruct,

    // special syntax for generics.
    MySimpleGeneric<T> where <T>,
    MyGenericType<T, F> where <T, F: HashObject>,
    dyn MyGenericTrait<T> where <T: SomeTraitBound>,
}
```
