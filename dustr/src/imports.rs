use crate::helpers::*;
use ::itertools::Itertools;

impl ::std::fmt::Display for crate::Imports {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|i| format!("import '{}';", i)).join("\n"))
    }
}

impl crate::Imports {
    pub fn from_module(m: &crate::Module, pkg: &str) -> Self {
        let mut res = Self(vec!["dart:ffi".to_owned()]);
        res = res + m.functions.iter().map(|ifn| Self::from_function(ifn, pkg, &m.crate_name)).sum();
        res = res + m.enums.iter().map(|en| Self::from_data(en, pkg, &m.crate_name)).sum();
        res = res + m.structs.iter().map(|st| Self::from_data(st, pkg, &m.crate_name)).sum();
        res
    }

    fn from_data(data: &::ffishim::Data, pkg: &str, crate_name: &str) -> Self {
        Self(vec![format!("package:{}/dylib.dart", pkg)]) + match &data.data {
            ::darling::ast::Data::Struct(fields) => Self::from_fields(&fields, pkg, crate_name),
            ::darling::ast::Data::Enum(variants) => variants.iter().map(|v| {
                Self::from_fields(&v.fields, pkg, crate_name)
            }).sum(),
        }
    }

    fn from_fields(
        fields: &::darling::ast::Fields<::ffishim::Field>,
        pkg: &str,
        crate_name: &str,
    ) -> Self {
        Self(fields.iter().filter_map(|f| if f.opaque {
            None
        } else {
            Some(crate::types::switch(&f.ty).imports(&f.ty, pkg, crate_name))
        }).flatten().collect())
    }

    fn from_function(ifn: &::syn::ItemFn, pkg: &str, crate_name: &str) -> Self {
        let mut imports = vec![format!("package:{}/dylib.dart", &pkg)];

        let ty = ret_ty(ifn);
        imports.extend(crate::types::switch(&ty).imports(&ty, pkg, crate_name));

        imports.extend(ifn.sig.inputs.iter().map(|arg| {
            let ty = arg_ty(arg);
            crate::types::switch(&ty).imports(&ty, pkg, crate_name)
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
