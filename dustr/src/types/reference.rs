use ::syn::*;

/// The behavior of simple, read-only &references.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Reference(_) = sty {
            true
        } else {
            false
        }
    }
    fn imports(&self, sty: &Type, pkg: &str, crate_name: &str) -> Vec<String> {
        let sty = inner_type(sty);
        crate::types::switch(sty).imports(sty, pkg, crate_name)
    }
    fn name(&self, sty: &Type) -> String {
        let sty = inner_type(sty);
        crate::types::switch(sty).name(sty)
    }

    fn ffi(&self, sty: &Type) -> String {
        let sty = inner_type(sty);
        crate::types::switch(sty).ffi(sty)
    }
    fn native(&self, sty: &Type) -> String {
        let sty = inner_type(sty);
        crate::types::switch(sty).native(sty)
    }

    fn native_to_ffi(&self, sty: &Type, expr: String) -> String {
        let sty = inner_type(sty);
        crate::types::switch(sty).native_to_ffi(sty, expr)
    }
    fn ffi_to_native(&self, sty: &Type, expr: String) -> String {
        let sty = inner_type(sty);
        crate::types::switch(sty).ffi_to_native(sty, expr)
    }
}

fn inner_type(sty: &Type) -> &Type {
    if let Type::Reference(r) = sty {
        &*r.elem
    } else {
        panic!("unexpected non-reference type")
    }
}
