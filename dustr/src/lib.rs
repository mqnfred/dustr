//! `dustr` is as a binary that parses rust code to generate its dart bindings. The rust code must
//! be marked using procedural macros from the [ffishim_derive][1] library.
//!
//! With dustr, you can call this rust code:
//!
//! ```ignore
//! #[ffishim_function]
//! fn hello(s: String) -> String {
//!     format!("Hello, {}!", s)
//! }
//! ```
//!
//! from dart:
//!
//! ```Dart
//! import 'package:hello/hello.dart';
//!
//! void main() {
//!     var greeting = hello("fred");
//!     print("${greeting}");
//! }
//! ```
//!
//! For more context, please take a look at the [README.md][2].
//!
//! [1]: https://github.com/mqnfred/ffishim
//! [2]: https://github.com/mqnfred/dustr

#[macro_use]
extern crate serde_derive;
#[macro_use]
mod helpers;

/// A rust module (hierarchy), built by parsing a crate.
///
/// The `Module` can be parsed from a full crate, a simple .rs file, or from a `mod` block in
/// a file. When parsing from a crate, all three methods are used whenever required.
#[derive(Debug)]
pub struct Module {
    name: String,
    crate_name: String,

    structs: Vec<::ffishim::Data>,
    enums: Vec<::ffishim::Data>,
    functions: Vec<::syn::ItemFn>,

    subs: Vec<Module>,
}
mod module;

/// The dart package built from rust sources.
///
/// This package is nothing but a set of [`Library`][1] objects with a name. You can build the dart
/// package by calling its `build` method and providing the folder in which you would like the dart
/// package to be built.
///
/// [1]: struct.Library.html
#[derive(Debug)]
pub struct Package {
    name: String,
    libraries: Vec<Library>,
}
mod package;

/// The dart equivalent of a rust [`Module`][1].
///
/// `Library`s map 1:1 with rust modules. You can initialize a `Library` by calling its `try_from`
/// static method and providing it with the rust module you want to create bindings for.
///
/// [1]: struct.Module.html
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

/// Imports helps manage imports (remove duplicates etc.)
///
/// A set of imports can be built from many other objects in dustr. You can get the imports list
/// for a rust module, from a data structure, from a function, fields... All imports from all items
/// in a library are then bubbled up to be used by the [`Library`][2] object, which will finally
/// generate them.
///
/// The `Imports` structure ensures nothing is imported twice.
///
/// [1]: struct.Module.html
/// [2]: struct.Library.html
#[derive(Debug)]
pub struct Imports(Vec<String>);
mod imports;

/// The dart representation of a rust structure.
///
/// `Struct` is created from a struct [`ffishim::Data`][1] object. It contains all metadata
/// necessary to generate the final bindings for this structure (field names, types, constructor,
/// destructor, ...)
///
/// [1]: https://docs.rs/ffishim/0.1.2/ffishim/struct.Data.html
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

/// The dart representation of a rust enum.
///
/// `Enum` is created from an enum [`ffishim::Data`][1]. It contains all metadata necessary to
/// generate the final bindings for this enum (variants, field names, types, constructors,
/// destructors...)
///
/// [1]: https://docs.rs/ffishim/0.1.2/ffishim/struct.Data.html
#[derive(Debug)]
pub struct Enum {
    name: String,
    base_struct: Struct,
    variant_structs: Vec<Struct>,
    variant_tags: Vec<String>,
}
mod r#enum;

/// A dart ffi function binding declaration.
///
/// `Function` is created from the rust `syn::ItemFn` to expose in Dart. Upon calling `fmt`, this
/// will generate the declaration of the function across the ffi boundary in dart as a global
/// variable. That function will never be called directly, it'll instead be called by our generated
/// [`Wrapper`][1].
///
/// [1]: struct.Wrapper.html
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

/// A dart wrapper around our function declarations.
///
/// `Wrapper` is created from the rust `syn::ItemFn` we want to wrap in Dart. The wrapper calls the
/// inner function after sanitizing input/outputs. It will also transform any `Result::Err`
/// variants returned into a Dart exception. This `Wrapper` contains all metadata necessary for
/// generation of the wrapper.
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
