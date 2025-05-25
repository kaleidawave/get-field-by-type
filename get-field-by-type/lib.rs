#![doc = include_str!("./README.md")]

pub use get_field_by_type_derive::GetFieldByType;

pub trait GetFieldByType<T> {
    /// Get the value of a field that has type T
    fn get(&self) -> &T;
}

// For `Box`, `Rc`, etc
impl<T, U> GetFieldByType<T> for U
where
    U: std::ops::Deref,
    U::Target: GetFieldByType<T>,
{
    fn get(&self) -> &T {
        std::ops::Deref::deref(self).get()
    }
}
