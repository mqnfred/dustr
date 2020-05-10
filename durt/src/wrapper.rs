use ::heck::MixedCase;
use ::itertools::Itertools;
use crate::helpers::*;
use crate::types::Behavior;

impl ::std::fmt::Display for crate::Wrapper {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if let Some(result_cast) = &self.result_cast {
            write!(f,
                "{ret} {name}({field_decls}) {{\n\
                \tvar result = {func_name}({field_conversions});\n\
                \tvar output = result.ref.unpack<{result_cast}>().value;\n\
                \tfreeResult(result);\n\
                \treturn {ret_conversion};\n\
                }}",
                ret = self.ret,
                name = self.name,
                field_decls = self.fields.iter().map(|(n, ty)| format!("{} {}", ty, n)).join(", "),

                func_name = self.func_name,
                field_conversions = self.field_conversions.iter().join(", "),
                ret_conversion = self.ret_conversion,

                result_cast = result_cast,
            )
        } else {
            write!(f,
                "{ret} {name}({field_decls}) {{\n\
                \tvar output = {func_name}({field_conversions});\n\
                \treturn {ret_conversion};\n\
                }}",
                ret = self.ret,
                name = self.name,
                field_decls = self.fields.iter().map(|(n, ty)| format!("{} {}", ty, n)).join(", "),

                func_name = self.func_name,
                field_conversions = self.field_conversions.iter().join(", "),
                ret_conversion = self.ret_conversion,
            )
        }
    }
}

impl ::std::convert::TryFrom<&::syn::ItemFn> for crate::Wrapper {
    type Error = ::anyhow::Error;
    fn try_from(ifn: &::syn::ItemFn) -> ::anyhow::Result<Self> {
        Ok(Self{
            name: ifn.sig.ident.to_string().to_mixed_case(),
            fields: ifn.sig.inputs.iter().map(|arg| {
                let ty = arg_ty(arg);
                (arg_ident(arg).to_string(), crate::types::switch(&ty).native(&ty))
            }).collect(),
            ret: { let ty = subtype(ret_ty(ifn).clone()); crate::types::switch(&ty).native(&ty) },

            func_name: format!("_{}", ifn.sig.ident),
            field_conversions: ifn.sig.inputs.iter().map(|arg| {
                let ty = arg_ty(arg);
                crate::types::switch(&ty).native_to_ffi(&ty, arg_ident(arg).to_string())
            }).collect(),
            ret_conversion: {
                let ty = subtype(ret_ty(ifn).clone());
                crate::types::switch(&ty).ffi_to_native(&ty, "output".to_owned())
            },

            result_cast: {
                let ty = ret_ty(ifn);
                if crate::types::Result.is(&ty) {
                    let subtype = subtype(ty.clone());
                    Some(crate::types::switch(&subtype).shim(&subtype))
                } else {
                    None
                }
            },
        })
    }
}
