import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'string.dart';

Pointer<Pointer<T>> optional_struct<T extends Struct>(T s) {
	if (s == null) {
		return Pointer<Pointer<T>>.fromAddress(0);
	} else {
		Pointer<Pointer<T>> opt = allocate(count: 1);
		opt.value = s.addressOf;
		return opt;
	}
}

Pointer<Pointer<Utf8>> optional_string(String s) {
	if (s == null) {
		return Pointer<Pointer<Utf8>>.fromAddress(0);
	} else {
		Pointer<Pointer<Utf8>> opt = allocate(count: 1);
		opt.value = stringToCString(s);
		return opt;
	}
}

Pointer<Uint32> optional_bool(bool b) {
	if (b == null) {
		return Pointer<Uint32>.fromAddress(0);
	} else {
		Pointer<Uint32> opt = allocate(count: 1);
		if (b) {
			opt.value = 1;
		} else {
			opt.value = 0;
		}
		return opt;
	}
}

Pointer<Float> optional_f32(double d) {
	if (d == null) {
		return Pointer<Float>.fromAddress(0);
	} else {
		Pointer<Float> opt = allocate(count: 1);
		opt.value = d;
		return opt;
	}
}

Pointer<Double> optional_f64(double d) {
	if (d == null) {
		return Pointer<Double>.fromAddress(0);
	} else {
		Pointer<Double> opt = allocate(count: 1);
		opt.value = d;
		return opt;
	}
}

Pointer<Uint8> optional_u8(int i) {
	if (i == null) {
		return Pointer<Uint8>.fromAddress(0);
	} else {
		Pointer<Uint8> opt = allocate(count: 1);
		opt.value = i;
		return opt;
	}
}

Pointer<Uint16> optional_u16(int i) {
	if (i == null) {
		return Pointer<Uint16>.fromAddress(0);
	} else {
		Pointer<Uint16> opt = allocate(count: 1);
		opt.value = i;
		return opt;
	}
}

Pointer<Uint32> optional_u32(int i) {
	if (i == null) {
		return Pointer<Uint32>.fromAddress(0);
	} else {
		Pointer<Uint32> opt = allocate(count: 1);
		opt.value = i;
		return opt;
	}
}

Pointer<Uint64> optional_u64(int i) {
	if (i == null) {
		return Pointer<Uint64>.fromAddress(0);
	} else {
		Pointer<Uint64> opt = allocate(count: 1);
		opt.value = i;
		return opt;
	}
}

Pointer<Int8> optional_i8(int i) {
	if (i == null) {
		return Pointer<Int8>.fromAddress(0);
	} else {
		Pointer<Int8> opt = allocate(count: 1);
		opt.value = i;
		return opt;
	}
}

Pointer<Int16> optional_i16(int i) {
	if (i == null) {
		return Pointer<Int16>.fromAddress(0);
	} else {
		Pointer<Int16> opt = allocate(count: 1);
		opt.value = i;
		return opt;
	}
}

Pointer<Int32> optional_i32(int i) {
	if (i == null) {
		return Pointer<Int32>.fromAddress(0);
	} else {
		Pointer<Int32> opt = allocate(count: 1);
		opt.value = i;
		return opt;
	}
}

Pointer<Int64> optional_i64(int i) {
	if (i == null) {
		return Pointer<Int64>.fromAddress(0);
	} else {
		Pointer<Int64> opt = allocate(count: 1);
		opt.value = i;
		return opt;
	}
}
