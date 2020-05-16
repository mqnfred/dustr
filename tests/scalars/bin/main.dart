import 'package:scalars/scalars.dart';

void main() {
	var a = add(2, 3);
	print("$a");

	var b = not(true);
	print("$b");
	var c = not(false);
	print("$c");

	var d = getNone();
	print("$d");
	var e = getSome(9);
	print("$e");
}
