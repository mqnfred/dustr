impl ::std::fmt::Display for crate::Enum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "")
    }
}

impl ::std::convert::TryFrom<&::ffishim::Data> for crate::Enum {
    type Error = ::anyhow::Error;
    fn try_from(d: &::ffishim::Data) -> ::anyhow::Result<Self> {
        Ok(Self{
            structs: unwrap_variants(d).iter().map(|variant| {
                crate::Struct::from_variant(&d.ident, variant)
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn unwrap_variants(d: &::ffishim::Data) -> &Vec<::ffishim::Variant> {
    match &d.data {
        ::darling::ast::Data::Struct(_) => panic!("cannot build enum out of struct"),
        ::darling::ast::Data::Enum(variants) => variants,
    }
}
