
pub trait Type {
    /// generate an Id when referenced in a field
    fn gen_type_to(buffer: &mut String);

    fn gen_type() -> String {
        let mut buffer = String::new();
        Self::gen_type_to(&mut buffer);
        buffer
    }
}

pub trait Interface {
    /// generate the whole typescript interface
    fn gen_interface_to(buffer: &mut String);

    fn gen_interface() -> String {
        let mut buffer = String::new();
        Self::gen_interface_to(&mut buffer);
        buffer
    }
}

impl Type for () {
    fn gen_type_to(buffer: &mut String) {
        buffer.push_str("null");
    }
}

impl Type for bool {
    fn gen_type_to(buffer: &mut String) {
        buffer.push_str("boolean");
    }
}

impl<T: Type> Type for Option<T> {
    fn gen_type_to(buffer: &mut String) {
        T::gen_type_to(buffer);
        buffer.push_str(" | null");
    }
}

macro_rules! impl_number {
    ($n:ty) => {
        impl Type for $n {
            fn gen_type_to(buffer: &mut String) {
                buffer.push_str("number");
            }
        }
    };
}

macro_rules! impl_string {
    ($n:ty) => {
        impl Type for $n {
            fn gen_type_to(buffer: &mut String) {
                buffer.push_str("string");
            }
        }
    };
}

macro_rules! impl_array {
    ($n:ty) => {
        impl<T: Type> Type for $n {
            fn gen_type_to(buffer: &mut String) {
                T::gen_type_to(buffer);
                buffer.push_str("[]");
            }
        }
    };
}

macro_rules! impl_map {
    ($n:ty) => {
        impl<T: Type, U: Type> Type for $n {
            fn gen_type_to(buffer: &mut String) {
                buffer.push_str("Record<");
                T::gen_type_to(buffer);
                buffer.push(',');
                U::gen_type_to(buffer);
                buffer.push('>');
            }
        }
    };
}

macro_rules! impl_wrapper {
    ($n:ty) => {
        impl<T: Type> Type for $n {
            fn gen_type_to(buffer: &mut String) {
                T::gen_type_to(buffer);
            }
        }
    };
}

macro_rules! impl_tuple {
    ($($t:tt),*) => {
        impl<$($t: Type),*> Type for ($($t),*) {
            fn gen_type_to(buffer: &mut String) {
                buffer.push_str("[");
                $(
                    $t::gen_type_to(buffer);
                    buffer.push_str(",");
                )*
                buffer.push_str("]");
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
impl_number!(isize);
impl_number!(f32);
impl_number!(f64);

impl_string!(char);
impl_string!(str);
impl_string!(String);

impl_array!([T]);
impl_array!(Vec<T>);
impl_array!(std::collections::VecDeque<T>);

impl_map!(std::collections::HashMap<T,U>);
impl_map!(std::collections::BTreeMap<T,U>);

impl_wrapper!(Box<T>);
impl_wrapper!(std::sync::Arc<T>);
impl_wrapper!(std::sync::Mutex<T>);
impl_wrapper!(std::sync::RwLock<T>);

impl_tuple!(T1, T2);
impl_tuple!(T1, T2, T3);
impl_tuple!(T1, T2, T3, T4);
impl_tuple!(T1, T2, T3, T4, T5);
impl_tuple!(T1, T2, T3, T4, T5, T6);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);

