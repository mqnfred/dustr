import 'package:strings/strings.dart';

void main() {
	var addr = Address.build("hello", "world", "!");
	print("${addr.line} ${addr.city} ${addr.country}");

	addr = setAddressLine(addr, "coucou");
	print("${addr.line} ${addr.city} ${addr.country}");

	// TODO: the dart side leaks this string in the wrapper
	// function. this is a real bug that needs fixing. would
	// likely imply wrapping the string on the dart side
	var line = getAddressLine(addr);
	print("${line}");
}
