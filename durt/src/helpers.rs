use ::syn::*;
use crate::types::Behavior;

pub fn is_same_id(path: &Path, id: &str) -> bool {
    id == &path.segments.last().expect("always >1 segments").ident.to_string()
}

pub fn arg_ty(arg: &FnArg) -> &Type {
    match arg {
        FnArg::Receiver(_) => panic!("functions with 'self' not supported"),
        FnArg::Typed(pat) => pat.ty.as_ref(),
    }
}

pub fn arg_ident(arg: &FnArg) -> &Ident {
    match arg {
        FnArg::Receiver(_) => panic!("functions with 'self' not supported"),
        FnArg::Typed(pat) => match pat.pat.as_ref() {
            ::syn::Pat::Ident(pati) => &pati.ident,
            _ => panic!("non-ident arg patterns not supported"),
        },
    }
}

pub fn ret_ty(ifn: &ItemFn) -> Type {
    match &ifn.sig.output {
        ::syn::ReturnType::Default => parse_quote! {
            Result<(), ::ffishim::library::Error>
        },
        ::syn::ReturnType::Type(_, ty) => if crate::types::Result.is(ty.as_ref()) {
            *ty.clone()
        } else {
            parse_quote! { Result<#ty, ::ffishim::library::Error> }
        },
    }
}

pub fn subtype(ty: Type) -> Type {
    let path = if let Type::Path(tp) = ty {
        tp.path
    } else {
        panic!("only normal type of kind typepath supported")
    };

    let segment = path.segments.into_iter().last().expect("always >0 elements in type path");

    let mut args = match segment.arguments {
        PathArguments::AngleBracketed(arguments) => arguments.args.into_iter(),
        PathArguments::None => panic!("expecting subtype"),
        _ => panic!("only bracketed arguments are supported"),
    };

    match args.next().expect("expecting subtype") {
        GenericArgument::Type(ty) => ty,
        _ => panic!("only the type arguments are supported"),
    }
}

pub fn name_of_field(idx: u32, ident: &Option<::syn::Ident>) -> String {
    if let Some(ident) = ident {
        ident.to_string()
    } else {
        alpha_idx(idx).to_owned()
    }
}

pub fn alpha_idx(idx: u32) -> &'static str {
    match idx {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => panic!("does not support more than 20 tuple struct fields"),
    }
}
