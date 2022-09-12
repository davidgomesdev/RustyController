import 'package:flutter_test/flutter_test.dart';
import 'package:rusty_controller/main.dart';

void main() {
  setUp(() {
    setupDependencies();
  });

  testWidgets('Base screen builds', (tester) async {
    await tester.pumpWidget(const BaseScreen());
  });
}
