#[macro_use]
extern crate serde_derive;
#[macro_use]
mod helpers;

#[derive(Debug)]
pub struct Module {
    name: String,
    root: String,

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
    enums: Vec<Enum>,
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

    declarations: Vec<FieldDeclaration>,
    getters: Vec<Getter>,

    new_func: Option<Function>,
    new_method: Option<NewMethod>,
    free_func: Option<Function>,
    free_method: Option<FreeMethod>,
}
#[derive(Debug)]
struct NewMethod {
    struct_name: String,
    args: Vec<(String, String)>, // native type, field name
    call_expr: String,
}
#[derive(Debug)]
struct FreeMethod {
    func_name: String,
    cast_to: Option<String>,
}
#[derive(Debug)]
struct FieldDeclaration {
    annotation: Option<String>,
    ffi_type: String,
    name: String,
}
#[derive(Debug)]
struct Getter {
    native_type: String,
    name: String,
    expr: String,
}
mod r#struct;

#[derive(Debug)]
pub struct Enum {
    name: String,
    base_struct: Struct,
    variant_structs: Vec<Struct>,
    variant_tags: Vec<String>,
}
mod r#enum;

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
