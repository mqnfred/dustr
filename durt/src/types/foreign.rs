use ::syn::*;

/// Any unknown type's behavior, assumed to implement an ffi shim themselves.
///
/// If a type is unknown (meaning it did not match any pre-defined `Behavior`,) we consider it by
/// default to be a `Foreign` type: a type defined by the user that itself implements the ffi shim.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, _: &Type) -> bool {
        true
    }

    fn ffi(&self, _sty: &Type, _cs: super::CallSite) -> crate::FFIType { todo!() }
    fn native(&self, _sty: &Type, _cs: super::CallSite) -> crate::NativeType { todo!() }

    fn native_to_ffi(&self, _sty: &Type, _expr: String) -> String { todo!() }
    fn ffi_to_native(&self, _sty: &Type, _expr: String) -> String { todo!() }
}
