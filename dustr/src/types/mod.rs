use ::once_cell::sync::Lazy;
use ::syn::*;

/// The behavior of a `Type` as needed to generate its dart code.
pub trait Behavior: Sync + Send {
    fn is(&self, sty: &Type) -> bool;
    fn imports(&self, sty: &Type, pkg: &str, crate_name: &str) -> Vec<String>;
    fn name(&self, sty: &Type) -> String;

    fn shim(&self, sty: &Type) -> String { self.ffi(sty) }
    fn ffi(&self, sty: &Type) -> String;
    fn native(&self, sty: &Type) -> String;

    fn native_to_ffi(&self, sty: &Type, expr: String) -> String;
    fn ffi_to_native(&self, sty: &Type, expr: String) -> String;
}

/// Switch over a given `Type` and return the associated `Behavior`.
///
/// This is an open-ended, c-style switch: if two different type behaviors' `is` method return
/// `true`, the first one in the list will win. You can order the type behaviors in the
/// `BEHAVIORS` vector.
pub fn switch<'a, 'b>(sty: &'a Type) -> &'b Box<dyn Behavior> {
    // TODO: give more context about which type we do not find the behavior of
    BEHAVIORS.iter().find(|tyb| tyb.is(sty)).expect("cannot find behavior for given type")
}

/// List of available, instantiated `Behavior`s.
static BEHAVIORS: Lazy<Vec<Box<dyn Behavior>>> = Lazy::new(|| {
    vec![
        // End-types
        Box::new(scalars::Behavior),
        Box::new(self::bool::Behavior),
        Box::new(duration::Behavior),
        Box::new(string::Behavior),

        // Parameterized types
        Box::new(option::Behavior),
        Box::new(result::Behavior),
        Box::new(vec::Behavior),
        Box::new(reference::Behavior),

        // Foreign/custom types implementing an ffi shim
        Box::new(foreign::Behavior),
    ]
});

// End-types
mod scalars; pub use scalars::Behavior as BehaviorScalars;
mod bool; pub use self::bool::Behavior as BehaviorBool;
mod duration; pub use duration::Behavior as BehaviorDuration;
mod string;

// Parameterized types
mod option;
mod result; pub use result::Behavior as BehaviorResult;
mod vec;
mod reference;

// Foreign/custom types implementing an ffi shim
mod foreign; pub use foreign::Behavior as BehaviorForeign;
