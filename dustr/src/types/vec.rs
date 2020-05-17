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
    fn imports(&self, sty: &Type, pkg: &str, crate_name: &str) -> Vec<String> {
        let subtype = subtype(sty.clone());
        let mut imports = crate::types::switch(&subtype).imports(&subtype, pkg, crate_name);
        imports.push(format!("package:{}/dustr/array.dart", pkg));
        imports
    }
    fn name(&self, _sty: &Type) -> String {
        panic!("option of array or array of option not supported");
    }

    fn ffi(&self, _sty: &Type) -> String {
        "Pointer<Array>".to_owned()
    }
    fn native(&self, sty: &Type) -> String {
        let subtype = subtype(sty.clone());
        let mut subtype_name = crate::types::switch(&subtype).name(&subtype);
        if let Some(r) = subtype_name.get_mut(0..1) {
            r.make_ascii_uppercase();
        }

        if subtype_name == "Struct" {
            format!("StructList<{}>", crate::types::switch(&subtype).native(&subtype))
        } else {
            format!("{}List", subtype_name)
        }
    }

    fn native_to_ffi(&self, _sty: &Type, expr: String) -> String {
        format!("{}.array", expr)
    }
    fn ffi_to_native(&self, sty: &Type, expr: String) -> String {
        format!("{}({})", self.native(sty), expr)
    }
}
