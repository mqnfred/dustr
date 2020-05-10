import 'package:bindings/scalars.dart';

void main() {
	var a = add(2, 3);
	print("$a");

	var gps1 = GPS.build(2.0, 3.0);
	print("${gps1.lat}");
	print("${gps1.lon}");
	gps1.free();

	/*
	var gps2 = GPS.build(5.0, 7.0);
	print("${gps2.lat}");
	gps2 = addToLatitude(gps2, 1.0);
	print("${gps2.lat}");
	print("${gps2.lon}");
	gps2.free();
	*/
}
