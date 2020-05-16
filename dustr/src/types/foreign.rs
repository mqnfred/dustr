use ::itertools::Itertools;
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
    fn imports(&self, sty: &Type, pkg: &str, crate_name: &str) -> Vec<String> {
        import_from_type(sty, pkg, crate_name).map(|i| vec![i]).unwrap_or_else(|| vec![])
    }
    fn name(&self, _sty: &Type) -> String { "struct".to_owned() }

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
}

fn import_from_type(sty: &Type, pkg: &str, crate_name: &str) -> Option<String> {
    if let Type::Path(tp) = sty {
        import_from_path(&tp.path, pkg, crate_name)
    } else {
        panic!("only normal type of kind typepath supported")
    }
}

fn import_from_path(path: &Path, pkg: &str, crate_name: &str) -> Option<String> {
    let mut segments = path.segments.iter().peekable();
    let first = segments.next().expect(">0 segments always");
    let first_str = first.ident.to_string();

    if first_str == "super" {
        panic!("type paths using the 'super' keyword are forbidden")
    } else if first_str == "crate" {
        if let Some(path) = segments_to_import_path(segments) {
            Some(format!("package:{}/{}/{}.dart", pkg, crate_name, path))
        } else {
            Some(format!("package:{}/{}.dart", pkg, crate_name))
        }
    } else if segments.peek().is_some() {
        Some(format!(
            "package:{}/{}.dart",
            pkg,
            segments_to_import_path(::std::iter::once(first).chain(segments)).expect(""),
        ))
    } else {
        // no imports needed, same file
        None
    }
}

fn segments_to_import_path<'a, I: Iterator<Item=&'a PathSegment>>(segments: I) -> Option<String> {
    let mut elements: Vec<String> = segments.map(|seg| {
        let ident = seg.ident.to_string();
        if ident == "super" {
            "..".to_string()
        } else if ident == "crate" {
            panic!("cannot use keyword 'crate' in the middle of a type path")
        } else {
            ident
        }
    }).collect();

    if elements.len() > 2 {
        Some(elements.chunks_exact(elements.len()-1).flatten().join("/"))
    } else if elements.len() == 2 {
        Some(elements.remove(0))
    } else {
        None
    }
}
