
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

macro_rules! impl_string {
    ($n:ty) => {
        impl Type for $n {
            fn generate(buffer: &mut String) {
                buffer.push_str("string");
            }
        }
    };
}

macro_rules! impl_array {
    ($n:ty) => {
        impl<T: Type> Type for $n {
            fn generate(buffer: &mut String) {
                T::generate(buffer);
                buffer.push_str("[]");
            }
        }
    };
}

macro_rules! impl_map {
    ($n:ty) => {
        impl<T: Type, U: Type> Type for $n {
            fn generate(buffer: &mut String) {
                buffer.push_str("Record<");
                T::generate(buffer);
                buffer.push(',');
                U::generate(buffer);
                buffer.push('>');
            }
        }
    };
}

macro_rules! impl_wrapper {
    ($n:ty) => {
        impl<T: Type> Type for $n {
            fn generate(buffer: &mut String) {
                T::generate(buffer);
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

impl<T: Type> Type for Option<T> {
    fn generate(buffer: &mut String) {
        T::generate(buffer);
        buffer.push_str(" | null");
    }
}

