use ::itertools::Itertools;

impl ::std::fmt::Display for crate::Enum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
            "enum {name}Tag {{\n\
            \t{variant_tags}\n\
            }}\n\
            {base_struct}\n\
            {variant_structs}\n",
            name = self.name,
            variant_tags = self.variant_tags.join(",\n\t"),
            base_struct = self.base_struct,
            variant_structs = self.variant_structs.iter().join("\n"),
        )
    }
}

impl ::std::convert::TryFrom<&::ffishim::Data> for crate::Enum {
    type Error = ::anyhow::Error;
    fn try_from(d: &::ffishim::Data) -> ::anyhow::Result<Self> {
        let mut variant_structs = Vec::<crate::Struct>::default();
        let mut variant_tags = vec![];
        let mut getters = vec![crate::Getter::enum_tag(&d.ident)];

        for v in unwrap_variants(d) {
            variant_structs.push(crate::Struct::from_variant(&d.ident, v)?);
            getters.push(crate::Getter::enum_variant(&d.ident, &v.ident));
            variant_tags.push(v.ident.to_string());
        }

        let free_func = crate::Function::free_function(&d.ident)?;
        let free_func_name = free_func.name.clone();

        Ok(Self{
            name: d.ident.to_string(),
            base_struct: crate::Struct{
                name: d.ident.to_string(),

                declarations: vec![crate::FieldDeclaration::enum_tag()],
                getters,

                new_func: None,
                new_method: None,
                free_func: Some(free_func),
                free_method: Some(crate::FreeMethod::from_func_name(free_func_name, None)),
            },
            variant_structs,
            variant_tags,
        })
    }
}

fn unwrap_variants(d: &::ffishim::Data) -> &Vec<::ffishim::Variant> {
    match &d.data {
        ::darling::ast::Data::Struct(_) => panic!("cannot build enum out of struct"),
        ::darling::ast::Data::Enum(variants) => variants,
    }
}
