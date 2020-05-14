import 'package:strings/strings.dart';

void main() {
	var addr = Address.build("hello", "world", "!");
	print("${addr.line} ${addr.city} ${addr.country}");
	addr = setAddressLine(addr, "coucou");
	print("${addr.line} ${addr.city} ${addr.country}");
	addr.free();
}
