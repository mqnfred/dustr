use ::syn::*;

pub fn ret_ty(ifn: &ItemFn) -> Option<&Type> {
    match &ifn.sig.output {
        ::syn::ReturnType::Default => None,
        ::syn::ReturnType::Type(_, ty) => Some(ty.as_ref()),
    }
}

pub fn arg_ty(arg: &FnArg) -> &Type {
    match arg {
        FnArg::Receiver(_) => panic!("functions with 'self' not supported"),
        FnArg::Typed(pat) => pat.ty.as_ref(),
    }
}

pub fn is_same_id(path: &Path, id: &str) -> bool {
    id == &path.segments.last().expect("always >1 segments").ident.to_string()
}
