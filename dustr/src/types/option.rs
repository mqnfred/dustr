use ::syn::*;
use crate::helpers::*;

/// The std lib's `Option` type behavior.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Option")
        } else {
            false
        }
    }
    fn imports(&self, _sty: &Type, pkg: &str, _crate_name: &str) -> Vec<String> {
        vec![format!("package:{}/dustr/option.dart", pkg)]
    }
    fn name(&self, _sty: &Type) -> String {
        panic!("option of option forbidden")
    }

    fn shim(&self, sty: &Type) -> String {
        let subtype = subtype(sty.clone());
        format!("Pointer<{}>", crate::types::switch(&subtype).shim(&subtype))
    }
    fn ffi(&self, sty: &Type) -> String {
        let subtype = subtype(sty.clone());
        format!("Pointer<{}>", crate::types::switch(&subtype).shim(&subtype))
    }
    fn native(&self, sty: &Type) -> String {
        let subtype = subtype(sty.clone());
        crate::types::switch(&subtype).native(&subtype)
    }

    fn native_to_ffi(&self, sty: &Type, expr: String) -> String {
        let subtype = subtype(sty.clone());
        format!("optional_{}({})", type_name_from_path(&subtype), expr)
    }
    fn ffi_to_native(&self, sty: &Type, expr: String) -> String {
        let subtype = subtype(sty.clone());
        format!(
            "({}.address == 0) ? null : {}",
            expr.clone(),
            crate::types::switch(&subtype).ffi_to_native(&subtype, format!("{}.value", expr)),
        )
    }
}
