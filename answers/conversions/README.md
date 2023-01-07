# Type conversions

Rust offers a multitude of ways to convert a value of a given type into another type.

The simplest form of type conversion is a type cast expression. It is denoted with the binary operator `as`. For instance, `println!("{}", 1 + 1.0);` would not compile, since `1` is an integer while `1.0` is a float. However, `println!("{}", 1 as f32 + 1.0)` should compile. The exercise [`using_as`](using_as.rs) tries to cover this.

Rust also offers traits that facilitate type conversions upon implementation. These traits can be found under the [`convert`](https://doc.rust-lang.org/std/convert/index.html) module.
The traits are the following:
- `From` and `Into` covered in [`from_into`](from_into.rs)
- `TryFrom` and `TryInto` covered in [`try_from_into`](try_from_into.rs)
- `AsRef` and `AsMut` covered in [`as_ref_mut`](as_ref_mut.rs)

Furthermore, the `std::str` module offers a trait called [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) which helps with converting strings into target types via the `parse` method on strings. If properly implemented for a given type `Person`, then `let p: Person = "Mark,20".parse().unwrap()` should both compile and run without panicking.

These should be the main ways ***within the standard library*** to convert data into your desired types.

## Further information

These are not directly covered in the book, but the standard library has a great documentation for it.
- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)

## AsRef & AsMut
as_ref() obtain a reference
as_mut() obtain a mutable reference
```rust
pub trait AsRef<T>
where
    T: ?Sized,
{
    fn as_ref(&self) -> &T;
}
// 获得本身的引用
```
T: AsRef<str> + AsMut<str>

## FromStr 
FromStr trait 能够实现 str.parse::<Type>() 方法
将字符串转为对应的类型

## try_from
能够将一个类型转为另一个类型
Color::try_from((1, 2, 3)) 将元组转为 Color 结构体，只需实现对应的方法
同时也会自动实现 try_into 指定对应的返回值

## as
as 可以用来做类型分配 eg:: 32 as i32 在计算的过程中会当成 i32 类型
记住as还可以用来重命名导入 use std::{mem as memory, net as network};


 