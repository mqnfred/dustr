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
    pub fn new_from_data(_libname: String, _d: &::ffishim::Data) -> ::anyhow::Result<Self> {
        // fill libname
        // get ffi_name from name new_data
        // iterate args, harvest name and types' .native() and .ffi() implems
        // ret type should be ffi.Pointer<d>?
        todo!()
    }

    pub fn free_from_data(_libname: String, _d: &::ffishim::Data) -> ::anyhow::Result<Self> {
        // fill libname
        // get ffi_name from name free_data
        // iterate args, harvest name and types' .native() and .ffi() implems
        // no ret type (Void?)
        todo!()
    }
}
