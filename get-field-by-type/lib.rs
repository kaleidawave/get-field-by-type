#![doc = include_str!("./README.md")]

pub use get_field_by_type_derive::GetFieldByType;

pub trait GetFieldByType<T> {
    /// Get the value of a field that has type T
    fn get(&self) -> &T;
}
