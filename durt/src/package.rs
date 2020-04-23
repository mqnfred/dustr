impl crate::Package {
    fn build(&self, dest: ::std::path::PathBuf) -> ::anyhow::Result<()> {
        // TODO: how to handle re-generation on top? remove and re-create dest?
        //
        // serde = "1.0"
        // serde_yaml = "0.8"
        //
        // create dest/pubspec.yaml and set:
        //  name
        //  dependencies:ffi
        //  environment: dart sdk version
        // create the lib/ folder in dest
        // call libraries.build(dest/lib)
        todo!()
    }
}
