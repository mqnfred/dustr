use crate::helpers::*;
use ::itertools::Itertools;

impl ::std::fmt::Display for crate::Imports {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|i| format!("import '{}';", i)).join("\n"))
    }
}

impl crate::Imports {
    pub fn from_module(pkg: &str, m: &crate::Module) -> Self {
        let mut res = Self(vec!["dart:ffi".to_owned()]);
        res = res + m.functions.iter().map(|ifn| Self::from_function(pkg, ifn)).sum();
        res
    }

    fn from_function(pkg: &str, ifn: &::syn::ItemFn) -> Self {
        let mut imports = vec![format!("package:{}/dylib.dart", &pkg)];

        let ty = ret_ty(ifn);
        imports.extend(crate::types::switch(&ty).imports(&ty, pkg));

        imports.extend(ifn.sig.inputs.iter().map(|arg| {
            let ty = arg_ty(arg);
            crate::types::switch(&ty).imports(&ty, pkg)
        }).flatten());

        let mut s = Self(imports);
        s.remove_duplicates();
        s
    }

    fn remove_duplicates(&mut self) {
        self.0.sort_by(|l, r| l.cmp(r));
        self.0.dedup();
    }
}

impl ::std::ops::Add<crate::Imports> for crate::Imports {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self.0.extend(other.0);
        self.remove_duplicates();
        self
    }
}

impl ::std::iter::Sum<crate::Imports> for crate::Imports {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Self(vec![]), ::std::ops::Add::add)
    }
}
