# DUSTR

`dustr` is a binary that generates dart bindings for a rust crate's library.

## How it works

In order for a rust data structure or function to be callable from those dart
bindings, it needs to be tagged with the [ffishim][1] family of procedural
macros.

Those procedural macros generate a FFI-compatible (C ABI) API around the
original data structure/function. This is necessary because many basic rust
types (`String`, `Option`, `Vec`, ...) do not respect the C ABI.

`dustr` will parse the rust crate and look for those bindings to find data
structures and methods that are "dart-compatible", and will generate dart
bindings for those.

## Examples

You can find examples of `dustr`'s behavior by looking at the [`tests`][1]
folder. The structure of the tests are as follow:

 - `src/lib.rs`: the rust library to expose
 - `Cargo.toml`: manifest of the rust library (defines cdylib, etc..)
 - `pubspec.yaml`: manifest of dart binary (defines dep on generated bindings)
 - `bin/main.dart`: the dart code that uses this rust library
 - `expected_output`: contains the output expected from running the C program

Every test folder is a stand-alone app. For example, you could:

 - Run `make` to test all of them
 - Inspect the generated bindings code in the `target/bindings` hierarchy
 - Tweak a test or two to try out behavior and rerun `make` to test things out
 - Use them as templates to create your own working rust <-> dart integration

## C ABI Disclaimer

Because dart ffi support is still in alpha, it cannot quite consume the C ABI
just yet. For example, [it does not support nested structs][2], and [structures
cannot be passed by value to functions][3]. For this reason, the ffishim crate
we use does not generate C-ABI code exactly, but a bastard version consumable
by the dart ffi.

## TODO/Limitations

This crate is still in beta. It is not fit for production use yet.

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
[2]: https://github.com/dart-lang/sdk/issues/37271
[3]: https://github.com/dart-lang/sdk/issues/41062
