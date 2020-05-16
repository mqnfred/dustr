use ::heck::{MixedCase,SnakeCase};
use ::itertools::Itertools;
use crate::helpers::*;
use crate::types::Behavior;

impl ::std::fmt::Display for crate::Struct {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
            "class {name} extends Struct {{\n\
                \t{declarations}\n\
                \n\
                \t{getters}\n\
                \n\
                \t{new_method}\n\
                \t{free_method}\n\
            }}\n\
            {new_func}\n\
            {free_func}\n\
            ",
            name = self.name,

            declarations = self.declarations.iter().join("\n\t"),
            getters = self.getters.iter().join("\n\t"),

            new_method = tostring_if_some(&self.new_method),
            free_method = tostring_if_some(&self.free_method),

            new_func = tostring_if_some(&self.new_func),
            free_func = tostring_if_some(&self.free_func),
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
        let mut s = Self::from_fields(&concat_idents(enum_name, &variant.ident), &variant.fields)?;

        s.declarations.insert(0, crate::FieldDeclaration::enum_tag());
        s.getters.insert(0, crate::Getter::enum_tag(enum_name));
        s.getters.insert(1, crate::Getter::enum_repr(enum_name));

        s.free_func = None;
        s.free_method = Some(crate::FreeMethod::from_func_name(
            format!("free{}", enum_name),
            Some(enum_name.to_string()),
        ));

        Ok(s)
    }

    pub fn from_fields(
        name: &::syn::Ident,
        fields: &::darling::ast::Fields<::ffishim::Field>,
    ) -> ::anyhow::Result<Self> {
        let declarations = crate::FieldDeclaration::from_fields(fields);
        let getters = crate::Getter::from_fields(fields);

        let mut new_func = None;
        let mut new_method = None;
        if fields.iter().all(|f| !f.opaque) {
            new_func = Some(generate_new_func(&name, fields)?);
            new_method = Some(crate::NewMethod::new(
                name.to_string(),
                new_func.as_ref().expect("new_func was just initialized").name.clone(),
                fields,
            ));
        }

        let free_func = crate::Function::free_function(&name)?;
        let free_method = crate::FreeMethod::from_func_name(free_func.name.clone(), None);

        Ok(Self{
            name: name.to_string(),
            declarations,
            getters,

            new_func,
            new_method,
            free_func: Some(free_func),
            free_method: Some(free_method),
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

pub fn generate_new_func(
    name: &::syn::Ident,
    fields: &::darling::ast::Fields<::ffishim::Field>,
) -> ::anyhow::Result<crate::Function> {
    let shim_name = format!("new_{}", name.to_string().to_snake_case());

    Ok(crate::Function{
        lib_name: "dylib".to_owned(),

        name: shim_name.to_mixed_case(),
        field_types: fields.iter().map(|f| {
            crate::types::switch(&f.ty).ffi(&f.ty)
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

impl ::std::fmt::Display for crate::NewMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
            "factory {struct_name}.build({args}) => {call_expr};",
            struct_name = self.struct_name,
            args = self.args.iter().map(|(t, n)| format!("{} {}", t, n)).join(", "),
            call_expr = self.call_expr,
        )
    }
}
impl crate::NewMethod {
    fn new(
        struct_name: String,
        new_func: String,
        fields: &::darling::ast::Fields<::ffishim::Field>,
    ) -> Self {
        Self{
            struct_name,
            args: fields.iter().enumerate().map(|(idx, f)| (
                crate::types::switch(&f.ty).native(&f.ty),
                name_of_field(idx as u32, &f.ident),
            )).collect(),
            call_expr: format!(
                "{}({}).ref",
                new_func,
                fields.iter().enumerate().map(|(idx, f)| {
                    let name = name_of_field(idx as u32, &f.ident);
                    crate::types::switch(&f.ty).native_to_ffi(&f.ty, name)
                }).join(", "),
            ),
        }
    }
}

impl ::std::fmt::Display for crate::FreeMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if let Some(cast_to) = self.cast_to.as_ref() {
            write!(f,
               "void free() {{ {name}(Pointer<{cast_to}>.fromAddress(addressOf.address)); }}",
               name = self.func_name,
               cast_to = cast_to,
            )
        } else {
            write!(f, "void free() {{ {name}(addressOf); }}", name = self.func_name)
        }
    }
}
impl crate::FreeMethod {
    pub fn from_func_name(func_name: String, cast_to: Option<String>) -> Self {
        Self{func_name, cast_to}
    }
}

impl ::std::fmt::Display for crate::FieldDeclaration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
            "{annotation} {ffi_type} {name};",
            annotation = tostring_if_some(&self.annotation),
            ffi_type = self.ffi_type,
            name = self.name,
        )
    }
}
impl crate::FieldDeclaration {
    pub fn enum_tag() -> Self {
        Self{
            annotation: Some("@Uint16()".to_owned()),
            ffi_type: "int".to_owned(),
            name: "_tag".to_owned(),
        }
    }

    fn from_fields(fields: &::darling::ast::Fields<::ffishim::Field>) -> Vec<Self> {
        fields.iter().enumerate().map(|(idx, f)| {
            let name = format!("_{}", name_of_field(idx as u32, &f.ident));
            if f.opaque {
                Self{annotation: None, ffi_type: "Pointer<Void>".to_owned(), name}
            } else {
                Self{
                    annotation: annotation(&f.ty),
                    ffi_type: crate::types::switch(&f.ty).ffi(&f.ty),
                    name,
                }
            }
        }).collect()
    }
}

impl ::std::fmt::Display for crate::Getter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
            "{native_type} get {name} => {expr};",
            native_type = self.native_type,
            name = self.name,
            expr = self.expr,
        )
    }
}
impl crate::Getter {
    fn from_fields(fields: &::darling::ast::Fields<::ffishim::Field>) -> Vec<Self> {
        fields.iter().enumerate().filter_map(|(idx, f)| {
            if f.opaque {
                None
            } else {
                let behavior = crate::types::switch(&f.ty);
                let getter_name = name_of_field(idx as u32, &f.ident);
                let field_name = format!("_{}", getter_name);
                Some(Self{
                    native_type: behavior.native(&f.ty),
                    name: getter_name,
                    expr: behavior.ffi_to_native(&f.ty, field_name),
                })
            }
        }).collect()
    }

    pub fn enum_tag(enum_name: &::syn::Ident) -> Self {
        Self{
            native_type: format!("{}Tag", enum_name),
            name: "tag".to_owned(),
            expr: format!("{}Tag.values[_tag]", enum_name),
        }
    }

    pub fn enum_variant(enum_ident: &::syn::Ident, variant_ident: &::syn::Ident) -> Self {
        let fullname = concat_idents(&enum_ident, &variant_ident);
        let expr = format!("Pointer<{}>.fromAddress(addressOf.address).ref", fullname);
        Self{
            native_type: fullname.to_string(),
            name: variant_ident.to_string(),
            expr,
        }
    }

    fn enum_repr(enum_ident: &::syn::Ident) -> Self {
        Self{
            native_type: enum_ident.to_string(),
            name: "repr".to_owned(),
            expr: format!("Pointer<{}>.fromAddress(addressOf.address).ref", enum_ident),
        }
    }
}
