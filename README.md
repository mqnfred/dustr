# DUSTR

With `dustr`, you can call this rust code:

```rust
#[ffishim_function]
fn hello(s: String) -> String {
    format!("Hello, {}!", s)
}
```

from dart:

```dart
import 'package:hello/hello.dart';

void main() {
    var greeting = hello("fred");
    print("${greeting}");
}
```

`dustr` is a binary that parses rust code to generate its dart bindings. The
rust code must be marked using the [ffishim_derive][1] procedural macros from
the [ffishim][4] library.

These procedural macros generate an FFI-compatible API around the original data
structure/function. This is necessary because many basic rust types (`String`,
`Option`, `Vec`, ...) do not respect the C ABI.

## Install & usage

Now, suppose we want to reproduce our first example. We need make, cargo and
the dart sdk. We install dustr using cargo:

```sh
export PATH=$PATH:$HOME/.cargo/bin
cargo install dustr
```

We also create our hello crate which we will mark as C dynamic library (cdylib,
to generate a `.so` shared object in the `rusthello/target/debug` directory.)

```sh
cargo new --lib rusthello --name hello
cat >>rusthello/Cargo.toml <<EOF
ffishim = "0.1.1"
ffishim_derive = "0.1.1"

[lib]
crate-type = ["cdylib"]
EOF
cat >rusthello/src/lib.rs <<EOF
#[macro_use]
extern crate ffishim_derive;

#[ffishim_library]
#[ffishim_function]
fn hello(s: String) -> String {
    format!("Hello, {}!", s)
}
EOF
cargo build --manifest-path=rusthello/Cargo.toml
ls rusthello/target/debug/libhello.so
```

We took the opportunity to add the code (with a healthy dose more plumbing this
time) and build the library. Now let's step into the dart side...

```sh
dustr --dest darthello --name hello rusthello/
cd darthello; pub get; cd -
```

The `dustr` command will create the dart package containing the bindings to the
rusthello library. `pub get` pulls in any dependencies. We'll now set up the
dart app which will use our bindings:

```sh
mkdir -p dartapp/bin
cat >dartapp/bin/main.dart <<EOF
import 'package:hello/hello.dart';

void main() {
    var greeting = hello("fred");
    print("\${greeting}");
}
EOF
cat >dartapp/pubspec.yaml <<EOF
---
name: app
dependencies:
  hello:
    path: ../darthello
environment:
  sdk: ">=2.0.0 <3.0.0"
EOF
cd dartapp; pub get; cd -
```

Now we can run the app. Don't forget to provide the rust library:

```sh
LD_LIBRARY_PATH=rusthello/target/debug dart dartapp/bin/main.dart
```

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

[1]: https://docs.rs/ffishim_derive/0.1.1/ffishim_derive
[2]: https://github.com/dart-lang/sdk/issues/37271
[3]: https://github.com/dart-lang/sdk/issues/41062
[4]: https://github.com/mqnfred/ffishim
