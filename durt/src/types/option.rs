use ::syn::*;
use crate::helpers::*;

/// The std lib's `Option` type behavior.
pub struct Behavior;

impl super::Behavior for Behavior {
    fn is(&self, sty: &Type) -> bool {
        if let Type::Path(tp) = sty {
            is_same_id(&tp.path, "Option")
        } else {
            false
        }
    }
}
