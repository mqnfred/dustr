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

static NUMBER_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("f32", "::ffishim::library::libc::c_float");
    m.insert("f64", "::ffishim::library::libc::c_double");
    m.insert("u8", "::ffishim::library::libc::c_char");
    m.insert("u16", "::ffishim::library::libc::c_ushort");
    m.insert("u32", "::ffishim::library::libc::c_uint");
    m.insert("u64", "::ffishim::library::libc::c_ulong");
    m.insert("usize", "::ffishim::library::libc::size_t");
    m.insert("i8", "::ffishim::library::libc::c_schar");
    m.insert("i16", "::ffishim::library::libc::c_short");
    m.insert("i32", "::ffishim::library::libc::c_int");
    m.insert("i64", "::ffishim::library::libc::c_long");
    m.insert("isize", "::ffishim::library::libc::ssize_t");
    m
});

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            NUMBER_TYPES.keys().any(|t| {
                ::syn::parse_str::<::syn::Path>(t).unwrap() == tp.path
            })
        } else {
            false
        }
    }

    fn ffi(&self, _sty: &Type, _cs: super::CallSite) -> crate::FFIType { todo!() }
    fn native(&self, _sty: &Type, _cs: super::CallSite) -> crate::NativeType { todo!() }

    fn native_to_ffi(&self, _sty: &Type, _expr: String) -> String { todo!() }
    fn ffi_to_native(&self, _sty: &Type, _expr: String) -> String { todo!() }
}
