use ::once_cell::sync::Lazy;

/// The behavior of a `Type` as needed to generate its dart code.
pub trait Behavior: Sync + Send {
    fn is(&self, sty: &::syn::Type) -> bool;

    fn shim(&self, sty: &::syn::Type) -> ::std::string::String { self.ffi(sty) }
    fn ffi(&self, sty: &::syn::Type) -> ::std::string::String;
    fn native(&self, sty: &::syn::Type) -> ::std::string::String;

    fn annotation(&self, sty: &::syn::Type) -> ::std::option::Option<::std::string::String>;

    fn native_to_ffi(&self, sty: &::syn::Type, expr: ::std::string::String) -> ::std::string::String;
    fn ffi_to_native(&self, sty: &::syn::Type, expr: ::std::string::String) -> ::std::string::String;

    // TODO: the String/Option collision is getting old in this file...
    fn imports(&self, sty: &::syn::Type, pkg: &str) -> Vec<::std::string::String>;
}

/// Switch over a given `Type` and return the associated `Behavior`.
///
/// This is an open-ended, c-style switch: if two different type behaviors' `is` method return
/// `true`, the first one in the list will win. You can order the type behaviors in the
/// `BEHAVIORS` vector.
pub fn switch<'a, 'b>(sty: &'a ::syn::Type) -> &'b Box<dyn Behavior> {
    // TODO: give more context about which type we do not find the behavior of
    BEHAVIORS.iter().find(|tyb| tyb.is(sty)).expect("cannot find behavior for given type")
}

/// List of available, instantiated `Behavior`s.
static BEHAVIORS: Lazy<Vec<Box<dyn Behavior>>> = Lazy::new(|| {
    vec![
        // End-types
        Box::new(Scalars),
        Box::new(String),

        // Parameterized types
        Box::new(Option),
        Box::new(Result),
        Box::new(Vector),

        // Foreign/custom types implementing an ffi shim
        Box::new(Foreign),
    ]
});

// End-types
mod scalars;
pub use scalars::Behavior as Scalars;
mod string;
pub use string::Behavior as String;

// Parameterized types
mod option;
pub use option::Behavior as Option;
mod result;
pub use result::Behavior as Result;
mod vec;
pub use vec::Behavior as Vector;

// Foreign/custom types implementing an ffi shim
mod foreign;
pub use foreign::Behavior as Foreign;
