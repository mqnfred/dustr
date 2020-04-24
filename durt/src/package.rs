use ::std::convert::TryFrom;

impl crate::Package {
    pub fn build(&self, _dest: ::std::path::PathBuf) -> ::anyhow::Result<()> {
        // call _remove_package
        //
        // serde = "1.0"
        // serde_yaml = "0.8"
        //
        // create dest/pubspec.yaml and set:
        //  name
        //  dependencies: ffi, durt itself
        //  environment: dart sdk version
        // create the lib/ folder in dest
        // call libraries.build(dest/lib)
        //
        // call _initialize_dependencies
        todo!()
    }
}

impl crate::Package {
    pub fn new(name: String, crate_paths: Vec<::std::path::PathBuf>) -> ::anyhow::Result<Self> {
        Ok(crate::Package{
            name,
            libraries: crate_paths.into_iter().map(|path| {
                crate::Library::try_from(crate::Module::from_crate(path)?)
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn _initialize_dependencies(_dest: ::std::path::PathBuf) -> ::anyhow::Result<()> {
    // use ::std::process::Command
    // https://doc.rust-lang.org/std/process/struct.Command.html
    // and then execute, potentially need to cd?
    todo!()
}

fn _remove_package(_dest: ::std::path::PathBuf) -> ::anyhow::Result<()> {
    // just rmdir dest
    todo!()
}
