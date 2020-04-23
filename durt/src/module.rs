impl crate::Module {
    pub fn from_crate(_path: ::std::path::PathBuf) -> ::anyhow::Result<Self> {
        // parse crate name from manifest
        // Module::new(crate_name, path/src/lib.rs)
        todo!()
    }

    pub fn from_file(_name: String, _path: ::std::path::PathBuf) -> ::anyhow::Result<Self> {
        // use syn to parse_file
        // call from_items
        todo!()
    }

    pub fn from_itemmod(
        _parent: ::std::path::PathBuf,
        _im: ::syn::ItemMod,
    ) -> ::anyhow::Result<Self> {
        // if itemmod is empty, find path of module (name/mod.rs or name.rs), call from_file
        // otherwise, iterate over items just like from_file
        todo!()
    }

    pub fn from_items(
        _name: String,
        _base: ::std::path::PathBuf,
        _items: Vec<::syn::Item>,
    ) -> ::anyhow::Result<Self> {
        // iterate over items:
        //  if is/ie/ifn: process_item and add to current module
        //  if mod: from_itemmod(dirname(path) or dirname(path)/name, itemmod) and add to modules
        todo!()
    }
}

fn _process_item_struct(_is: ::syn::ItemStruct) -> Option<::syn::DeriveInput> {
    todo!()
}

fn _process_item_enum(_ie: ::syn::ItemEnum) -> Option<::syn::DeriveInput> {
    todo!()
}

fn _module_name_from_path(_path: &::std::path::PathBuf) -> String {
    todo!()
}
