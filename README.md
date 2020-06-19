# DUSTR

Generate dart code to interact with rust [FFIShim][1] bindings.

## TODO/Limitations

### Bugs

 - Fix leaking of returned strings when not inside struct/enum
 - Fix leaking of returned options when not inside struct/enum

### Features

 - Adapt to potential name mangling in ffishim (to avoid duplicates)
 - Find equivalent dart type for usize/isize/char
 - Implement DateTime behavior

### Testing

 - Add a "complete" test/example situation
 - Re-organize tests by domain (find taxonomy)
 - Benchmark some use-cases

### Documentation

 - Write README.md introduction on what this crate does
 - Where to write binary documentation in doc.rs?

[1]: https://github.com/mqnfred/ffishim
