impl ::std::fmt::Display for crate::Wrapper {
    fn fmt(&self, _f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        // print a name, fields, ret signature on top
        // print call
        // if result_cast is_some(), then print unpack call with the cast
        // print return ret_conversion
        todo!()
    }
}

impl ::std::convert::TryFrom<&::ffishim::Function> for crate::Wrapper {
    type Error = ::anyhow::Error;
    fn try_from(_f: &::ffishim::Function) -> ::anyhow::Result<Self> {
        // fill libname
        // get ffi_name from ffishim module maybe? TODO
        // iterate args, harvest name and types' .native() and .ffi() implems
        // init ret using type's .native() and .ffi() implems
        // set conversion to ret type's .ffi_to_native()
        // if return type is result, then set the result cast with ret type's .ffi()
        todo!()
    }
}
