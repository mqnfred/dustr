# Durt

Generate dart code to interact with rust FFIShim bindings.

## TODO

### Implementation

 - Module initialization from paths to crates
 - Package building
 - Implement type behaviors for basic types:
   2. Strings
   3. Option
   5. Vec
 - Find equivalent dart type for usize/isize

### Testing

 - Make test framework call durt/initialize package
 - Add tests for all basic type behaviors
 - Add a "complete" test/example situation

### Documentation

 - Write README.md introduction on what this crate does
 - Where to write binary documentation in doc.rs?
