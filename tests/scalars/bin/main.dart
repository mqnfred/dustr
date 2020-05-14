import 'package:bindings/scalars.dart';

void main() {
	var a = add(2, 3);
	print("$a");

	var gps1 = GPS.build(2.0, 3.0);
	print("${gps1.lat}");
	print("${gps1.lon}");
	gps1.free();

	var gps2 = GPS.build(5.0, 7.0);
	print("${gps2.lat}");
	gps2 = addToLatitude(gps2, 1.0);
	print("${gps2.lat}");
	print("${gps2.lon}");
	gps2.free();

	var loc1 = LocationGPS.build(GPS.build(6.0, 9.0));
	print("${loc1.tag}");
	print("${loc1.zero}");
	print("${loc1.zero.lat}");
	print("${loc1.zero.lon}");
	print("${loc1.repr}");
	loc1.free();

	var loc2 = LocationAddress.build(94040);
	print("${loc2.tag}");
	print("${loc2.zip}");
	print("${loc2.repr}");
	loc2.free();

	var loc3 = LocationUnknown.build();
	print("${loc3.tag}");
	print("${loc3.repr}");
	loc3.free();
}
