impl crate::Library {
    fn _build(&self, _dest: ::std::path::PathBuf) -> ::anyhow::Result<()> {
        // print all of our items in dest/name.dart
        // if has sub-libraries:
        //  mkdir dest/name/
        //  for every sub-library, call lib.build(dest/name)
        todo!()
    }
}

impl ::std::convert::TryFrom<crate::Module> for crate::Library {
    type Error = ::anyhow::Error;
    fn try_from(_module: crate::Module) -> ::anyhow::Result<Self> {
        // inherit module name here,
        // initialize all items with module's data/functions
        // recursively call on submodule
        todo!()
    }
}
