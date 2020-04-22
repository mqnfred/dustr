use crate::helpers::*;
use ::syn::*;

/// The std lib's `Vec` type behavior, backed by ffishim library's `FFIVec`.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Vec")
        } else {
            false
        }
    }
}
