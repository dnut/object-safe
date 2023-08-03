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
Since `MyTrait` extends `Hash`, it is not object safe either, and `dyn MyTrait` is not a valid type. This crate offers a way to work around this limitation, so you can have object-safe traits whose objects implement object-unsafe traits such as `Hash`.

Instead of expressing `Hash` as the trait bound, express `HashObj` as the trait bound.
```rust
pub trait MyTrait: HashObj {}
```

You do not need to implement `HashObj`. It is automatically implemented for any type that implements `Hash`. Now, `dyn MyTrait` is object-safe. Add one line of code if you want `dyn MyTrait` to implement `Hash`:

```rust
impl_hash(dyn MyTrait);
```

Here are all the characteristics that come with HashObj:
- anything implementing `Hash` automatically implements `HashObj`
- `dyn HashObj` implements `Hash`.
- `Obj<T>` implements `Hash` for any `T` that derefs to something implementing `HashObj`.
- `impl_hash` can implement `Hash` for any type that implements `HashObj`, for example a trait object `dyn MyTrait` where `MyTrait` is a trait extending `HashObj`.

```rust
impl_hash! {
    // typical use, where MyTrait: HashObj
    dyn MyTrait,
    dyn AnotherTrait,

    // structs and enums are supported if they deref to
    // a target that implements HashObj or Hash.
    MyStruct,

    // special syntax for generics.
    MySimpleGeneric<T> where <T>,
    MyGenericType<T, F> where <T, F: HashObj>,
    dyn MyGenericTrait<T> where <T: SomeTraitBound>,

    // the actual impl for Obj
    Obj<T> where <T: Deref<Target=X>, X: HashObj + ?Sized>,
}
```
