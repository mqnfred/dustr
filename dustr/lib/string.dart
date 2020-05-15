import 'dart:convert';
import 'dart:ffi';
import 'package:ffi/ffi.dart';

Pointer<Utf8> stringToCString(String s) {
	List<int> units = Utf8Encoder().convert(s);
	Pointer<Int8> str = allocate(count: units.length + 1);
	for (var i = 0; i < units.length; ++i) {
		str.elementAt(i).value = units[i];
	}
	str.elementAt(units.length).value = 0;
	return str.cast();
}
