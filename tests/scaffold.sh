#!/bin/sh

if [ "${1}" = "-h" ] || [ "${1}" = "--help" ] || [ -z "${1}" ]; then
	echo "Please provide a test name: ${0} <desired_test_name>"
	echo "This will create a test in tests/<desired_test_name>"
	exit 127
else
	name="${1}"
	dir="tests/${name}"
fi

sed -i "s#^]\$#\t\"${dir}\",\n]#g" Cargo.toml
cargo new --lib ${dir}

cat >> ${dir}/Cargo.toml <<EOF
ffishim = "0.1.0"
ffishim_derive = "0.1.0"

[lib]
name = "${name}"
crate-type = ["cdylib"]
EOF

cat > ${dir}/pubspec.yaml <<EOF
name: ${name}_test
dependencies:
  ${name}:
    path: ../../target/bindings/${name}
environment:
  sdk: '>=2.0.0 <3.0.0'
EOF

mkdir ${dir}/bin
cat > ${dir}/bin/main.dart <<EOF
import 'package:${name}/${name}.dart';

void main() {
	print("Hello, world!");
}
EOF

cat > ${dir}/src/lib.rs <<EOF
#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]
fn lol() {}
EOF

cat > ${dir}/expected_output <<EOF
Hello, world!
EOF
