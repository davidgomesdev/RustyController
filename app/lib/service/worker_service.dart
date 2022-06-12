import 'dart:isolate';

import 'package:flutter/cupertino.dart';
import 'package:flutter/foundation.dart';

class WorkerService<T> {
  final ValueSetter<T> handler;
  late final Isolate isolate;

  WorkerService._(this.handler, this.isolate);

  static Future<WorkerService<T>> create<T>(void Function(T) handler,
      {T? initialMessage}) async {
    final isolate = await Isolate.spawn<T?>((msg) {
      if (msg != null) handler(msg);
    }, initialMessage);

    return WorkerService._(handler, isolate);
  }

  void send(T message) async {
    isolate.controlPort.send(message);
  }
}
