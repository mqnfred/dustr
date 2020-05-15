use crate::helpers::*;
use ::syn::*;

/// The std lib's `Result` type behavior.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Result")
        } else {
            false
        }
    }

    fn ffi(&self, _sty: &Type) -> String {
        "Pointer<Result>".to_owned()
    }

    fn native(&self, _sty: &Type) -> String {
        "Result".to_owned()
    }

    fn native_to_ffi(&self, _sty: &Type, _expr: String) -> String { todo!() }
    fn ffi_to_native(&self, _sty: &Type, _expr: String) -> String { todo!() }

    fn imports(&self, sty: &Type, pkg: &str) -> Vec<String> {
        let mut imports = vec!["dart:ffi".to_owned(), format!("package:{}/dustr/result.dart", pkg)];

        let subtype = subtype(sty.clone());
        imports.extend(crate::types::switch(&subtype).imports(&subtype, pkg));

        imports
    }
}
