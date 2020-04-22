import 'dart:io';

void alarm([String text]) {
  stdout.writeln(text ?? message);
}

String get message => 'Hello, world!';
