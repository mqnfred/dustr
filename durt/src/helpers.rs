use ::syn::*;

macro_rules! fmt {
    ($format_str:tt $(,$args:tt)*) => { format!(::indoc::indoc!($format_str), $($args),*) }
}

pub fn is_same_id(path: &Path, id: &str) -> bool {
    id == &path.segments.last().expect("always >1 segments").ident.to_string()
}
