import 'package:timing/timing.dart';

void main() {
	var dur = getDaysDurationInMs(5);
	print("${dur}");

	var wr1 = WorldRecord.build(dur);
	print("${wr1.time}");
	wr1.free();

	var wr2 = WorldRecord.build(Duration(milliseconds: 5));
	print("${wr2.time}");
	wr2.free();
}
