use ::heck::{MixedCase,SnakeCase};
use ::itertools::Itertools;
use crate::helpers::*;
use crate::types::Behavior;

impl ::std::fmt::Display for crate::Struct {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
            "class {name} extends Struct {{\n\
                \t{field_decls}\n\
                \n\
                \t{field_getters}\n\
                \n\
                \tfactory {name}.build({new_args}) => {new_call_expr};\n\
                \t{free_method}\n\
            }}\n\
            {new_func}\n\
            {free_func}\n\
            ",
            name = self.name,

            field_decls = self.field_decls.iter().map(|(a, t, n)| {
                a.as_ref().map(|a| {
                    format!("{} {} {};", a, t, n)
                }).unwrap_or_else(|| format!("{} {};", t, n))
            }).join("\n"),

            field_getters = self.field_getters.iter().map(|(nat, name, expr)| {
                format!("{} get {} => {};", nat, name, expr)
            }).join("\n"),

            new_args = self.new_args.iter().map(|(t, n)| format!("{} {}", t, n)).join(", "),
            new_call_expr = self.new_call_expr,
            free_method = self.free_method,

            free_func = self.free_func,
            new_func = self.new_func,
        )
    }
}

impl crate::Struct {
    pub fn from_data(data: &::ffishim::Data) -> ::anyhow::Result<Self> {
        Self::from_fields(&data.ident, unwrap_fields(data))
    }

    pub fn from_variant(
        enum_name: &::syn::Ident,
        variant: &::ffishim::Variant,
    ) -> ::anyhow::Result<Self> {
        Self::from_fields(&concat_idents(enum_name, &variant.ident), &variant.fields)
    }

    pub fn from_fields(
        name: &::syn::Ident,
        fields: &::darling::ast::Fields<::ffishim::Field>,
    ) -> ::anyhow::Result<Self> {
        let field_decls = fields.iter().enumerate().map(|(idx, f)| (
                annotation(&f.ty),
                crate::types::switch(&f.ty).ffi(&f.ty),
                format!("_{}", name_of_field(idx as u32, &f.ident)),
        )).collect();
        let field_getters = fields.iter().enumerate().map(|(idx, f)| {
            let behavior = crate::types::switch(&f.ty);
            let getter_name = name_of_field(idx as u32, &f.ident);
            let field_name = format!("_{}", getter_name);
            (behavior.native(&f.ty), getter_name, behavior.ffi_to_native(&f.ty, field_name))
        }).collect();

        let new_func = generate_new(&name, fields)?;
        let new_args = fields.iter().enumerate().map(|(idx, f)| (
            crate::types::switch(&f.ty).native(&f.ty),
            name_of_field(idx as u32, &f.ident),
        )).collect();
        let new_call_expr = format!(
            "{}({}).ref",
            new_func.name,
            fields.iter().enumerate().map(|(idx, f)| {
                let name = name_of_field(idx as u32, &f.ident);
                crate::types::switch(&f.ty).native_to_ffi(&f.ty, name)
            }).join(", "),
        );

        let free_func = crate::Function::free_function(&name)?;
        let free_method = format!("void free() {{ {}(addressOf); }}", free_func.name);

        Ok(Self{
            name: name.to_string(),
            field_decls,
            field_getters,

            new_func,
            new_args,
            new_call_expr,

            free_func,
            free_method,
        })
    }
}

fn annotation(sty: &::syn::Type) -> Option<String> {
    if crate::types::BehaviorScalars.is(sty) {
        Some(format!("@{}()", crate::types::BehaviorScalars.shim(sty)))
    } else {
        None
    }
}

fn generate_new(
    name: &::syn::Ident,
    fields: &::darling::ast::Fields<::ffishim::Field>,
) -> ::anyhow::Result<crate::Function> {
    let shim_name = format!("new_{}", name.to_string().to_snake_case());

    Ok(crate::Function{
        lib_name: "dylib".to_owned(),

        name: shim_name.to_mixed_case(),
        field_types: fields.iter().map(|f| {
            crate::types::switch(&f.ty).native(&f.ty)
        }).collect(),
        ret_type: format!("Pointer<{}>", name),

        shim_name,
        shim_field_types: fields.iter().map(|f| crate::types::switch(&f.ty).shim(&f.ty)).collect(),
        shim_ret_type: format!("Pointer<{}>", name),
    })
}

fn unwrap_fields(d: &::ffishim::Data) -> &::darling::ast::Fields<::ffishim::Field> {
    match &d.data {
        ::darling::ast::Data::Struct(fields) => fields,
        ::darling::ast::Data::Enum(_) => panic!("cannot build struct out of enum"),
    }
}
