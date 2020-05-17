import 'package:arrays/arrays.dart';

void main() {
	//var arr1 = newU64Array(2, 5);
	//print("${arr1[4]}");
	//print("${arr1[1]}");

	var crowd = createCrowd();
	print(crowd);
	print(crowd.members.length);

	print('---');

	print(crowd.members[0]);
	print(crowd.members[0].tagline);
	print(crowd.members[0].points[0]);
	print(crowd.members[0].points[1]);
	print(crowd.members[0].points[2]);

	print('---');

	print(crowd.members[1]);
	print(crowd.members[1].tagline);
	print(crowd.members[1].points[0]);
	print(crowd.members[1].points[1]);
	print(crowd.members[1].points[2]);

	print('---');

	try {
		print(crowd.members[2]);
	} catch (e) {
		print(e);
	}
	try {
		print(crowd.members[-1]);
	} catch (e) {
		print(e);
	}
}
