#[macro_use]
extern crate serde_derive;
#[macro_use]
mod helpers;

#[derive(Debug)]
pub struct Module {
    name: String,
    datas: Vec<::ffishim::Data>,
    functions: Vec<::syn::ItemFn>,
    subs: Vec<Module>,
}
mod module;

#[derive(Debug)]
pub struct Package {
    name: String,
    local_durt_lib: Option<::std::path::PathBuf>,
    libraries: Vec<Library>,
}
mod package;

#[derive(Debug)]
pub struct Library {
    name: String,
    imports: Vec<String>, // TODO: String best type?
    functions: Vec<Function>,
    wrappers: Vec<Wrapper>,
    subs: Vec<Library>,
}
mod library;

#[derive(Debug)]
pub struct Struct {
    name: String,

    field_decls: Vec<(FFIType, String)>,
    field_getters: Vec<(NativeType, String, String)>, // native type, getter name, expr

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

    ffi_name: String,
    ffi_field_types: Vec<String>,
    ffi_ret_type: String,
}
mod function;

#[derive(Debug)]
pub struct Wrapper {
    name: String,
    fields: Vec<(NativeType, String)>,
    ret: NativeType,

    func_name: String,
    field_conversions: Vec<String>,
    ret_conversion: String,

    result_cast: Option<String>,
}
mod wrapper;

type NativeType = String;
type FFIType = String;

pub mod types;
