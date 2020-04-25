use crate::helpers::*;
use ::syn::*;

/// The std lib's `Vec` type behavior, backed by ffishim library's `FFIVec`.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Vec")
        } else {
            false
        }
    }

    fn ffi(&self, _sty: &Type, _cs: super::CallSite) -> crate::FFIType { todo!() }
    fn native(&self, _sty: &Type, _cs: super::CallSite) -> crate::NativeType { todo!() }

    fn native_to_ffi(&self, _sty: &Type, _expr: String) -> String { todo!() }
    fn ffi_to_native(&self, _sty: &Type, _expr: String) -> String { todo!() }
}
