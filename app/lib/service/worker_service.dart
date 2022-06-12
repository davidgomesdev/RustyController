import 'dart:isolate';

class WorkerService<T> {
  final Future<void> Function(T) handler;
  final SendPort _sendPort;

  WorkerService._(this.handler, this._sendPort);

  static Future<WorkerService<T>> create<T>(Future<void> Function(T) handler,
      {T? initialMessage}) async {
    final receivePort = ReceivePort();

    await Isolate.spawn<SendPort>((sendPort) async {
      final messagePort = ReceivePort();

      sendPort.send(messagePort.sendPort);

      await for (final msg in messagePort) {
        msg.handler(msg.message);
      }
    }, receivePort.sendPort);

    final sendPort = await receivePort.first;

    return WorkerService._(handler, sendPort);
  }

  void send(T message) async {
    _sendPort.send(_WorkerMessage(handler, message));
  }
}

class _WorkerMessage<T> {
  /// Dirty workaround due to this issue: https://github.com/dart-lang/sdk/issues/36983
  ///
  /// We can't simply call the handler in the `WorkerService` isolate function
  Future<void> Function(T) handler;
  T message;

  _WorkerMessage(this.handler, this.message);
}
