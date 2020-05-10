#[macro_use]
extern crate serde_derive;
#[macro_use]
mod helpers;

#[derive(Debug)]
pub struct Module {
    name: String,
    structs: Vec<::ffishim::Data>,
    enums: Vec<::ffishim::Data>,
    functions: Vec<::syn::ItemFn>,
    subs: Vec<Module>,
}
mod module;

#[derive(Debug)]
pub struct Package {
    name: String,
    libraries: Vec<Library>,
}
mod package;

#[derive(Debug)]
pub struct Library {
    name: String,
    imports: Imports,
    structs: Vec<Struct>,
    functions: Vec<Function>,
    wrappers: Vec<Wrapper>,
    subs: Vec<Library>,
}
mod library;

#[derive(Debug)]
pub struct Imports(Vec<String>);
mod imports;

#[derive(Debug)]
pub struct Struct {
    name: String,

    field_decls: Vec<(Option<String>, String, String)>, // type annotation, ffi type, field name
    field_getters: Vec<(String, String, String)>,       // native type, name, expr

    new_func: Function,
    free_func: Function,
}
mod r#struct;

#[derive(Debug)]
pub struct Function {
    lib_name: String,

    name: String,
    field_types: Vec<String>,
    ret_type: String,

    shim_name: String,
    shim_field_types: Vec<String>,
    shim_ret_type: String,
}
mod function;

#[derive(Debug)]
pub struct Wrapper {
    name: String,
    fields: Vec<(String, String)>,
    ret: String,

    func_name: String,
    field_conversions: Vec<String>,
    ret_conversion: String,

    result_cast: Option<String>,
}
mod wrapper;

pub mod types;
