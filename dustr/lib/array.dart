import 'dart:collection';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'string.dart';

class Array extends Struct {
	@Uint64()
	int length;
	Pointer<Void> data;
}

class StructList<T extends Struct> extends ListBase<T> {
	Pointer<Array> array;
	StructList(this.array);

	@override set length(int newLength) => throw Exception('native lists are readonly');
	@override void operator []=(int index, T value) => throw Exception('native lists are readonly');
	@override int get length => array.ref.length;
	@override T operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Uint64>.fromAddress(array.ref.data.address);
		int address = ptr.elementAt(index).value;
		return Pointer<T>.fromAddress(address).ref;
	}
}

class StringList extends ListBase<String> {
	Pointer<Array> array;
	StringList(this.array);

	@override set length(int newLength) => throw Exception('native lists are readonly');
	@override void operator []=(int index, String value) => throw Exception('native lists are readonly');
	@override int get length => array.ref.length;
	@override String operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Uint64>.fromAddress(array.ref.data.address);
		int address = ptr.elementAt(index).value;
		return Utf8.fromUtf8(Pointer<Utf8>.fromAddress(address));
	}
}

class BoolList extends ListBase<bool> {
	Pointer<Array> array;
	BoolList(this.array);

	@override set length(int newLength) => throw Exception('native lists are readonly');
	@override void operator []=(int index, bool value) => throw Exception('native lists are readonly');
	@override int get length => array.ref.length;
	@override bool operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Uint32>.fromAddress(array.ref.data.address);
		if (ptr.elementAt(index).value == 0) {
			return false;
		} else {
			return true;
		}
	}
}

class FloatList extends ListBase<double> {
	Pointer<Array> array;
	FloatList(this.array);

	@override set length(int newLength) => throw Exception('native lists are readonly');
	@override void operator []=(int index, double value) => throw Exception('native lists are readonly');
	@override int get length => array.ref.length;
	@override double operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Float>.fromAddress(array.ref.data.address);
		return ptr.elementAt(index).value;
	}
}

class DoubleList extends ListBase<double> {
	Pointer<Array> array;
	DoubleList(this.array);

	@override set length(int newLength) => throw Exception('native lists are readonly');
	@override void operator []=(int index, double value) => throw Exception('native lists are readonly');
	@override int get length => array.ref.length;
	@override double operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Double>.fromAddress(array.ref.data.address);
		return ptr.elementAt(index).value;
	}
}

class IntList extends ListBase<int> {
	Pointer<Array> array;
	IntList(this.array);

	@override set length(int newLength) => throw Exception('native lists are readonly');
	@override void operator []=(int index, int value) => throw Exception('native lists are readonly');
	@override int get length => array.ref.length;
	@override int operator [](int index) {
		throw Exception('implemented by children');
	}
}

class I8List extends IntList {
	I8List(Pointer<Array> array) : super(array);
	@override int operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Int8>.fromAddress(array.ref.data.address);
		return ptr.elementAt(index).value;
	}
}

class I16List extends IntList {
	I16List(Pointer<Array> array) : super(array);
	@override int operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Int16>.fromAddress(array.ref.data.address);
		return ptr.elementAt(index).value;
	}
}

class I32List extends IntList {
	I32List(Pointer<Array> array) : super(array);
	@override int operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Int32>.fromAddress(array.ref.data.address);
		return ptr.elementAt(index).value;
	}
}

class I64List extends IntList {
	I64List(Pointer<Array> array) : super(array);
	@override int operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Int64>.fromAddress(array.ref.data.address);
		return ptr.elementAt(index).value;
	}
}

class U8List extends IntList {
	U8List(Pointer<Array> array) : super(array);
	@override int operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Uint8>.fromAddress(array.ref.data.address);
		return ptr.elementAt(index).value;
	}
}

class U16List extends IntList {
	U16List(Pointer<Array> array) : super(array);
	@override int operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Uint16>.fromAddress(array.ref.data.address);
		return ptr.elementAt(index).value;
	}
}

class U32List extends IntList {
	U32List(Pointer<Array> array) : super(array);
	@override int operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Uint32>.fromAddress(array.ref.data.address);
		return ptr.elementAt(index).value;
	}
}

class U64List extends IntList {
	U64List(Pointer<Array> array) : super(array);
	@override int operator [](int index) {
		if (index < 0 || index >= length) {
			throw Exception('index out of bounds');
		}

		var ptr = Pointer<Uint64>.fromAddress(array.ref.data.address);
		return ptr.elementAt(index).value;
	}
}
