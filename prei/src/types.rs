use std::collections::HashMap;

pub trait Type {
    /// generate an Id when referenced in a field
    fn generate(buffer: &mut String);
}

pub trait Interface {
    /// generate the whole typescript interface
    fn generate() -> String;
}

impl Type for () {
    fn generate(buffer: &mut String) {
        buffer.push_str("null");
    }
}

impl Type for bool {
    fn generate(buffer: &mut String) {
        buffer.push_str("boolean");
    }
}

macro_rules! impl_number {
    ($n:ty) => {
        impl Type for $n {
            fn generate(buffer: &mut String) {
                buffer.push_str("number");
            }
        }
    };
}

impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(usize);
impl_number!(i8);
impl_number!(i16);
impl_number!(i32);
impl_number!(i64);

impl Type for String {
    fn generate(buffer: &mut String) {
        buffer.push_str("string");
    }
}

impl<T: Type> Type for [T] {
    fn generate(buffer: &mut String) {
        T::generate(buffer);
        buffer.push_str("[]");
    }
}

impl<T: Type> Type for Vec<T> {
    fn generate(buffer: &mut String) {
        T::generate(buffer);
        buffer.push_str("[]");
    }
}

impl<T: Type, U: Type> Type for HashMap<T,U> {
    fn generate(buffer: &mut String) {
        buffer.push_str("Record<");
        T::generate(buffer);
        buffer.push(',');
        U::generate(buffer);
        buffer.push('>');
    }
}

