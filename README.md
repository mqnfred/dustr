# Durt

Generate dart code to interact with rust FFIShim bindings.

## TODO

### Implementation

 - Module initialization from paths to crates
 - Package building
 - Select which type behaviors we need:
   1. .ffi() -> dart ffi type
   2. .native() -> dart native type
   3. .ffi_to_native() -> expression to transform a ffi type into a native one
   4. .native_to_ffi() -> expression to transform a native type into a ffi one
 - Find solution for scalars' FFI/Native "intermediary" representation
 - Implement type behaviors for basic types:
   1. Scalars (u32, u64, f32...)
   2. Strings
   3. Option
   4. Foreign
   5. Vec
 - How to represent enums/tagged unions properly in dart? protobuf `oneof`?

### Testing

 - Make test framework call durt/initialize package
 - Add tests for all basic type behaviors
 - Add a "complete" test/example situation

### Documentation

 - Write README.md introduction on what this crate does
 - Where to write binary documentation in doc.rs?
