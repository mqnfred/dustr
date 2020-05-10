use ::syn::*;
use crate::helpers::*;

/// The std lib's `Option` type behavior.
pub struct BehaviorOption;

impl super::Behavior for BehaviorOption {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Option")
        } else {
            false
        }
    }

    fn ffi(&self, _sty: &Type) -> String { todo!() }
    fn native(&self, _sty: &Type) -> String { todo!() }
    fn annotation(&self, _sty: &Type) -> Option<String> { todo!() }

    fn native_to_ffi(&self, _sty: &Type, _expr: String) -> String { todo!() }
    fn ffi_to_native(&self, _sty: &Type, _expr: String) -> String { todo!() }

    fn imports(&self, _sty: &Type, _pkg: &str) -> Vec<String> { todo!() }
}
