impl ::std::fmt::Display for crate::Function {
    fn fmt(&self, _f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        // fairly straightforward print-out of the Function fields
        todo!()
    }
}

impl ::std::convert::TryFrom<(String, ::ffishim::Function)> for crate::Function {
    type Error = ::anyhow::Error;
    fn try_from((_libname, _f): (String, ::ffishim::Function)) -> ::anyhow::Result<Self> {
        // fill libname
        // get ffi_name from ffishim module maybe? TODO
        // iterate args, harvest name and types' .native() and .ffi() implems
        // init ret using type's .native() and .ffi() implems
        todo!()
    }
}

impl crate::Function {
    pub fn new_from_data(_libname: String, _d: &::ffishim::Data) -> ::anyhow::Result<Self> {
        // fill libname
        // get ffi_name from name new_data
        // iterate args, harvest name and types' .native() and .ffi() implems
        // ret type should be ffi.Pointer<d>?
        todo!()
    }

    pub fn free_from_data(_libname: String, _d: &::ffishim::Data) -> ::anyhow::Result<Self> {
        // fill libname
        // get ffi_name from name free_data
        // iterate args, harvest name and types' .native() and .ffi() implems
        // no ret type (Void?)
        todo!()
    }
}
