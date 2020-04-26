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
ffishim = { path = "../../../../ffishim/ffishim" }
ffishim_derive = { path = "../../../ffishim/ffishim_derive" }

[lib]
name = "${name}"
crate-type = ["cdylib"]
EOF

cat > ${dir}/pubspec.yaml <<EOF
name: ${name}
dependencies:
  bindings:
    path: ../../target/bindings/scalars
  durt:
    path: ../../durt
environment:
  sdk: '>=2.0.0 <3.0.0'
EOF

mkdir ${dir}/lib
cat > ${dir}/lib/main.dart <<EOF
import 'package:durt/durt.dart';
import 'package:bindings/${name}.dart';

void main() {
	alarm();
}
EOF

cat > ${dir}/src/lib.rs <<EOF
#[macro_use]
extern crate ffishim_derive;
EOF

cat > ${dir}/expected_output <<EOF
Hello, world!
EOF
