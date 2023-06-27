# Get-field-by-type

[![crates.io badge](https://img.shields.io/crates/v/get-field-by-type?style=flat-square)](https://crates.io/crates/get-field-by-type)
[![docs.rs badge](https://img.shields.io/docsrs/get-field-by-type?style=flat-square)](https://docs.rs/get-field-by-type/latest)

Create a getter for a field based on its type. For example

```rust
use get_field_by_type::GetFieldByType;

#[derive(GetFieldByType)]
#[get_field_by_type_target(i32)]
enum Enum1 {
	A(i32, char),
	B(i32, bool),
	C(i32, String)
}

let x = Enum1::A(12, '!');
assert_eq!(*GetFieldByType::<i32>::get(&x), 12);
```

## Why not `impl &Self for Into<&T>`

Using `Into` on references for getting a field is a bit of hack. This is designed for getting the value for a field with a common type. Using a custom trait also means flexibility in the future

## Additional options and features

### When a struct of variant doesn't have a type, there are two behaviors
Either a compile time error (default)

Or a specified **statement** to evaluate. This could be a panic

```rust
use get_field_by_type::GetFieldByType;

#[derive(GetFieldByType)]
#[get_field_by_type_target(i32)]
#[get_field_no_type_behavior(panic!("could not find item");)]
enum Enum2 {
	A(i32),
	B(i32),
	C
}

assert_eq!(*GetFieldByType::<i32>::get(&Enum2::A(12)), 12);

let result = std::panic::catch_unwind(|| {
	let _value = *GetFieldByType::<i32>::get(&Enum2::C);
});
assert!(result.is_err());
```

or returning a constant value

```rust
use get_field_by_type::GetFieldByType;

#[derive(GetFieldByType)]
#[get_field_by_type_target(i32)]
#[get_field_no_type_behavior(return &0;)]
enum Enum2 {
	A(i32),
	B(i32),
	C
}

assert_eq!(*GetFieldByType::<i32>::get(&Enum2::A(12)), 12);
assert_eq!(*GetFieldByType::<i32>::get(&Enum2::C), 0);
```

### Recursive on unit variants

In the `AOrB` enum case, the derive macro can't find a `i32` on variant `AOrB::A`. However, as it is a unit variant, the implementation delegates it to the unit type.

```rust
use get_field_by_type::GetFieldByType;

#[derive(GetFieldByType)]
#[get_field_by_type_target(i32)]
struct A(pub i32);

#[derive(GetFieldByType)]
#[get_field_by_type_target(i32)]
struct B(pub i32);

#[derive(GetFieldByType)]
#[get_field_by_type_target(i32)]
enum AOrB {
	A(A),
	B(B)
}

let b = B(10);
assert_eq!(*GetFieldByType::<i32>::get(&b), 10);

let a_or_b = AOrB::B(b);
assert_eq!(*GetFieldByType::<i32>::get(&a_or_b), 10);
```

## Alternatives

For more manual field access, check out [getset](https://github.com/jbaublitz/getset). If looking for getting multiple fields, check out [getters-by-type](https://crates.io/crates/getters-by-type)
