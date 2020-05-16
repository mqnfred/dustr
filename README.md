# DUSTR

Generate dart code to interact with rust FFIShim bindings.

## TODO

### Implementation

 - Find equivalent dart type for usize/isize/char
 - Implement type behaviors for bool, option, vec, ...
 - Fix leaking of returned strings when not inside struct/enum

### Testing

 - Make test framework call dustr/initialize package
 - Add a "complete" test/example situation

### Documentation

 - Write README.md introduction on what this crate does
 - Where to write binary documentation in doc.rs?
