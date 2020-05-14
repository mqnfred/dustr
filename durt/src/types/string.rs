use crate::helpers::*;
use ::syn::*;

/// The std lib's `String` type behavior.
///
/// We currently use a `CString` from to ingest all `String` values. This might be unsafe if the string
/// was instantiated by the caller without using rust's instanciation mechanism. In that case, it
/// would be safe to use a `CStr`. See https://doc.rust-lang.org/std/ffi/struct.CString.html
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "String")
        } else {
            false
        }
    }

    fn ffi(&self, _sty: &Type) -> String {
        "Pointer<Utf8>".to_owned()
    }
    fn native(&self, _sty: &Type) -> String {
        "String".to_owned()
    }

    fn native_to_ffi(&self, _sty: &Type, expr: String) -> String {
        format!("stringToCString({})", expr)
    }
    fn ffi_to_native(&self, _sty: &Type, expr: String) -> String {
        format!("Utf8.fromUtf8({})", expr)
    }

    fn imports(&self, _sty: &Type, pkg: &str) -> Vec<String> {
        vec![
            "dart:ffi".to_owned(),
            "dart:convert".to_owned(),
            "package:ffi/ffi.dart".to_owned(),
            format!("package:{}/durt/string.dart", pkg),
        ]
    }
}
