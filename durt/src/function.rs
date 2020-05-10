use ::heck::{MixedCase,SnakeCase};
use ::itertools::Itertools;
use crate::helpers::*;
use crate::types::Behavior;

impl ::std::fmt::Display for crate::Function {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
            "final {ret_type} \
            Function({field_types}) {func_name} = \
            {lib_name}.lookup<NativeFunction<\
                {ffi_ret_type} Function({ffi_field_types}) \
            >>('{ffi_name}').asFunction();",
            lib_name = self.lib_name,

            ret_type = self.ret_type,
            field_types = self.field_types.iter().join(", "),
            func_name = self.name,

            ffi_ret_type = self.ffi_ret_type,
            ffi_field_types = self.ffi_field_types.iter().join(", "),
            ffi_name = self.ffi_name,
        )
    }
}

impl ::std::convert::TryFrom<&::syn::ItemFn> for crate::Function {
    type Error = ::anyhow::Error;
    fn try_from(f: &::syn::ItemFn) -> ::anyhow::Result<Self> {
        Ok(Self{
            lib_name: "dylib".to_owned(),

            name: format!("_{}", f.sig.ident),
            field_types: f.sig.inputs.iter().map(|arg| custom_type_ffi(arg_ty(arg))).collect(),
            ret_type: custom_type_ffi(&ret_ty(f)),

            ffi_name: format!("ffi_{}", f.sig.ident),
            ffi_field_types: f.sig.inputs.iter().map(|arg| {
                let ty = arg_ty(arg);
                crate::types::switch(&ty).ffi(&ty)
            }).collect(),
            ffi_ret_type: { let ty = ret_ty(f); crate::types::switch(&ty).ffi(&ty) },
        })
    }
}

/// This custom type switch is necessary because in functions' return types, all types except
/// scalars use their ffi version. Scalars, on the other hand, use their native one (int.)
fn custom_type_ffi(ty: &::syn::Type) -> String {
    if crate::types::Scalars.is(ty) {
        crate::types::Scalars.native(ty)
    } else {
        crate::types::switch(ty).ffi(ty)
    }
}

impl crate::Function {
    pub fn free_from_data(d: &::ffishim::Data) -> ::anyhow::Result<Self> {
        let ffi_name = format!("free_{}", d.ident.to_string().to_snake_case());

        Ok(Self{
            lib_name: "dylib".to_owned(),

            name: ffi_name.to_mixed_case(),
            // TODO: vvvvvvvvv should this behavior live somewhere else?
            field_types: vec![format!("Pointer<{}>", d.ident)],
            ret_type: "void".to_owned(),

            ffi_name,
            // TODO: vvvvvvvvv should this behavior live somewhere else?
            ffi_field_types: vec![format!("Pointer<{}>", d.ident)],
            ffi_ret_type: "Void".to_owned(),
        })
    }
}
