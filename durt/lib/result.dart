import 'dart:ffi';
import 'package:ffi/ffi.dart';

class Error implements Exception {
	final String message;

	Error(this.message);
	@override String toString() => 'Error: ${message}';
}

class Result extends Struct {
	Pointer<Utf8> message;
	Pointer<Void> payload;

	Pointer<T> unpack<T extends NativeType>() {
		if (message.address != 0) {
			throw Error(Utf8.fromUtf8(message));
		}
		return Pointer<T>.fromAddress(payload.address);
	}
}
