import 'package:structs/structs.dart';
import 'package:structs/structs/submod.dart';
import 'package:structs/structs/submod/subsubscuba.dart';

void main() {
	var loc1 = LocationGPS.build(
		GPS.build(
			Latitude.build(
				MyFloat.build(1.0)
			),
			2.0
		)
	).repr;
	print("${loc1.GPS.zero.lat.zero.val}");
}
