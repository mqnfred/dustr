use ::syn::*;
use crate::helpers::*;

/// Any unknown type's behavior, assumed to implement an ffi shim themselves.
///
/// If a type is unknown (meaning it did not match any pre-defined `Behavior`,) we consider it by
/// default to be a `Foreign` type: a type defined by the user that itself implements the ffi shim.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, _: &Type) -> bool {
        true
    }

    fn ffi(&self, sty: &Type) -> String {
        format!("Pointer<{}>", type_name_from_path(sty))
    }
    fn native(&self, sty: &Type) -> String {
        type_name_from_path(sty)
    }

    fn native_to_ffi(&self, _sty: &Type, expr: String) -> String {
        format!("{}.addressOf", expr)
    }
    fn ffi_to_native(&self, _sty: &Type, expr: String) -> String {
        format!("{}.ref", expr)
    }

    fn imports(&self, _sty: &Type, _pkg: &str) -> Vec<String> {
        vec![] // TODO: if path length >1, import matching file
    }
}
