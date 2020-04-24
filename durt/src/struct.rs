impl ::std::fmt::Display for crate::Struct {
    fn fmt(&self, _f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        todo!()
    }
}

impl ::std::convert::TryFrom<::ffishim::Data> for crate::Struct {
    type Error = ::anyhow::Error;
    fn try_from(_d: ::ffishim::Data) -> ::anyhow::Result<Self> {
        // use same name as data structure
        //
        // for every field, call its type's .ffi() and prepend its name with _, collect
        // for every field, .native(), collect name and .ffi_to_native() expr
        //
        // populate new_func and free_func using Function::{new,free}_from_data
        todo!()
    }
}
