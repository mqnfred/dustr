use ::once_cell::sync::Lazy;
use ::std::collections::HashMap;
use ::syn::*;

/// Builtin scalar behaviors: `f32`, `u32`, ...
///
/// The behavior for different scalars is shared into this object. Here is the list of scalars and
/// their libc equivalents:
///
///  - `f32`
///  - `f64`
///  - `u8`
///  - `u16`
///  - `u32`
///  - `u64`
///  - `usize`
///  - `i8`
///  - `i16`
///  - `i32`
///  - `i64`
///  - `isize`
pub struct Behavior;

static NATIVE_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("f32", "double");
    m.insert("f64", "double");
    m.insert("u8", "int");
    m.insert("u16", "int");
    m.insert("u32", "int");
    m.insert("u64", "int");
    //m.insert("usize", "int"); TODO un-handled usize/isize scalar types
    m.insert("i8", "int");
    m.insert("i16", "int");
    m.insert("i32", "int");
    m.insert("i64", "int");
    //m.insert("isize", "int"); TODO un-handled usize/isize scalar types
    m
});

#[allow(dead_code)]
static SHIM_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("f32", "Float");
    m.insert("f64", "Double");
    m.insert("u8", "Uint8");
    m.insert("u16", "Uint16");
    m.insert("u32", "Uint32");
    m.insert("u64", "Uint64");
    //m.insert("usize", "int"); TODO un-handled usize/isize scalar types
    m.insert("i8", "Int8");
    m.insert("i16", "Int16");
    m.insert("i32", "Int32");
    m.insert("i64", "Int64");
    //m.insert("isize", "int"); TODO un-handled usize/isize scalar types
    m
});

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            NATIVE_TYPES.keys().any(|t| {
                parse_str::<Path>(t).unwrap() == tp.path
            })
        } else {
            false
        }
    }

    fn shim(&self, sty: &Type) -> String {
        if let Type::Path(tp) = sty {
            let name = tp.path.get_ident().expect(".shim() with non-scalar type").to_string();
            SHIM_TYPES.get(name.as_str()).expect(".shim() with non-scalar type").to_string()
        } else {
            panic!("cannot call scalar .ffi() with non-scalar type");
        }
    }

    fn ffi(&self, sty: &Type) -> String {
        self.native(sty)
    }

    fn native(&self, sty: &Type) -> String {
        if let Type::Path(tp) = sty {
            let name = tp.path.get_ident().expect(".native() with non-scalar type").to_string();
            NATIVE_TYPES.get(name.as_str()).expect(".native() with non-scalar type").to_string()
        } else {
            panic!("cannot call scalar .native() with non-scalar type");
        }
    }

    fn native_to_ffi(&self, _sty: &Type, expr: String) -> String {
        expr
    }

    fn ffi_to_native(&self, _sty: &Type, expr: String) -> String {
        expr
    }

    fn imports(&self, _sty: &Type, _pkg: &str, _root: &str) -> Vec<String> {
        vec!["dart:ffi".to_owned()]
    }
}
