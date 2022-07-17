import 'dart:typed_data';

extension IPConverter on String {
  Uint8List convertAddressToBytes() {
    var address = List<int>.empty(growable: true);

    for (final octet in split('.')) {
      address.add(int.parse(octet));
    }

    return Uint8List.fromList(address);
  }
}
