use ::heck::{MixedCase,SnakeCase};
use ::itertools::Itertools;
use crate::helpers::*;

impl ::std::fmt::Display for crate::Function {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
            "final {ret_type} \
            Function({field_types}) {func_name} = \
            {lib_name}.lookup<NativeFunction<\
                {shim_ret_type} Function({shim_field_types}) \
            >>('{shim_name}').asFunction();",
            lib_name = self.lib_name,

            ret_type = self.ret_type,
            field_types = self.field_types.iter().join(", "),
            func_name = self.name,

            shim_ret_type = self.shim_ret_type,
            shim_field_types = self.shim_field_types.iter().join(", "),
            shim_name = self.shim_name,
        )
    }
}

impl ::std::convert::TryFrom<&::syn::ItemFn> for crate::Function {
    type Error = ::anyhow::Error;
    fn try_from(f: &::syn::ItemFn) -> ::anyhow::Result<Self> {
        Ok(Self{
            lib_name: "dylib".to_owned(),

            name: format!("_{}", f.sig.ident),
            field_types: f.sig.inputs.iter().map(|arg| ffi_type(arg_ty(arg))).collect(),
            ret_type: ffi_type(&ret_ty(f)),

            shim_name: format!("ffi_{}", f.sig.ident),
            shim_field_types: f.sig.inputs.iter().map(|arg| shim_type(&arg_ty(arg))).collect(),
            shim_ret_type: shim_type(&ret_ty(f)),
        })
    }
}

fn shim_type(ty: &::syn::Type) -> String {
    crate::types::switch(ty).shim(ty)
}

fn ffi_type(ty: &::syn::Type) -> String {
    crate::types::switch(ty).ffi(ty)
}

impl crate::Function {
    pub fn free_function(ident: &::syn::Ident) -> ::anyhow::Result<Self> {
        let shim_name = format!("free_{}", ident.to_string().to_snake_case());

        Ok(Self{
            lib_name: "dylib".to_owned(),

            name: shim_name.to_mixed_case(),
            // TODO: vvvvvvvvv should this behavior live somewhere else?
            field_types: vec![format!("Pointer<{}>", ident)],
            ret_type: "void".to_owned(),

            shim_name,
            // TODO: vvvvvvvvv should this behavior live somewhere else?
            shim_field_types: vec![format!("Pointer<{}>", ident)],
            shim_ret_type: "Void".to_owned(),
        })
    }
}
