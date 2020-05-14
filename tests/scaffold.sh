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
ffishim = { path = "../../../ffishim/ffishim" }
ffishim_derive = { path = "../../../ffishim/ffishim_derive" }

[lib]
name = "bindings"
crate-type = ["cdylib"]
EOF

cat > ${dir}/pubspec.yaml <<EOF
name: ${name}
dependencies:
  bindings:
    path: ../../target/bindings/${name}
environment:
  sdk: '>=2.0.0 <3.0.0'
EOF

mkdir ${dir}/bin
cat > ${dir}/bin/main.dart <<EOF
import 'package:bindings/${name}.dart';

void main() {
	print("Hello, world!");
}
EOF

cat > ${dir}/src/lib.rs <<EOF
#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]
EOF

cat > ${dir}/expected_output <<EOF
Hello, world!
EOF
