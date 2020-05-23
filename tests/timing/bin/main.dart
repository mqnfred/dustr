import 'package:timing/timing.dart';

void main() {
	var dur = getDaysDurationInMs(5);
	print("${dur}");
	var wr = WorldRecord.build(dur);
	print("${wr.time}");
	wr.free();
}
