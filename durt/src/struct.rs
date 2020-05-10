use ::heck::{MixedCase,SnakeCase};
use ::itertools::Itertools;
use crate::helpers::*;

impl ::std::fmt::Display for crate::Struct {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
            "class {name} extends Struct {{\n\
                \t{field_decls}\n\
                \n\
                \t{field_getters}\n\
            }}\n\
            {new_func}\n\
            {free_func}\n\
            ",
            name = self.name,
            field_decls = self.field_decls.iter().map(|(a, t, n)| {
                a.as_ref().map(|a| {
                    format!("{} {} {}", a, t, n)
                }).unwrap_or_else(|| format!("{} {}", t, n))
            }).join("\n"),
            field_getters = self.field_getters.iter().map(|(nat, name, expr)| {
                format!("{} get {} => {};", nat, name, expr)
            }).join("\n"),
            free_func = self.free_func,
            new_func = self.new_func,
        )
    }
}

impl ::std::convert::TryFrom<::ffishim::Data> for crate::Struct {
    type Error = ::anyhow::Error;
    fn try_from(d: ::ffishim::Data) -> ::anyhow::Result<Self> {
        let fields = unwrap_fields(&d);

        let field_decls = fields.iter().enumerate().map(|(idx, f)| {
            let behavior = crate::types::switch(&f.ty);
            (behavior.annotation(&f.ty), behavior.ffi(&f.ty), format!("_{}", name_of_field(idx as u32, &f.ident)))
        }).collect();
        let field_getters = fields.iter().enumerate().map(|(idx, f)| {
            let behavior = crate::types::switch(&f.ty);
            let name = name_of_field(idx as u32, &f.ident);
            (behavior.native(&f.ty), name.clone(), behavior.ffi_to_native(&f.ty, name))
        }).collect();
        let new_func = generate_new(&d)?;
        let free_func = crate::Function::free_from_data(&d)?;

        Ok(Self{name: d.ident.to_string(), field_decls, field_getters, new_func, free_func})
    }
}

fn generate_new(d: &::ffishim::Data) -> ::anyhow::Result<crate::Function> {
    let fields = unwrap_fields(d);
    let shim_name = format!("new_{}", d.ident.to_string().to_snake_case());

    Ok(crate::Function{
        lib_name: "dylib".to_owned(),

        name: shim_name.to_mixed_case(),
        field_types: fields.iter().map(|f| {
            crate::types::switch(&f.ty).native(&f.ty)
        }).collect(),
        ret_type: format!("Pointer<{}>", d.ident),

        shim_name,
        shim_field_types: fields.iter().map(|f| crate::types::switch(&f.ty).ffi(&f.ty)).collect(),
        shim_ret_type: format!("Pointer<{}>", d.ident),
    })
}

fn unwrap_fields(d: &::ffishim::Data) -> &::darling::ast::Fields<::ffishim::Field> {
    match &d.data {
        ::darling::ast::Data::Struct(fields) => fields,
        ::darling::ast::Data::Enum(_) => panic!("cannot build struct out of enum"),
    }
}
