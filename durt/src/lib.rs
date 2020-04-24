#[derive(Debug)]
pub struct Module {
    name: String,
    datas: Vec<::ffishim::Data>,
    functions: Vec<::ffishim::Function>,
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
    libname: String,

    name: String,
    ffi_name: String,
    fields: Vec<(NativeType, FFIType)>,
    ret: (NativeType, FFIType),
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

#[macro_use]
mod helpers;
