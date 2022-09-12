import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:network_info_plus/network_info_plus.dart';
import 'package:rusty_controller/bloc/discovery_bloc.dart';
import 'package:rusty_controller/extensions/network_extensions.dart';
import 'package:rusty_controller/main.dart';

const handshakeBeginPort = 31337;
const handshakeEndPort = 31338;
const handshakeRequest = "HeyoDearClient";
const handshakeResponse = "HelloDearRusty";

const handshakeTimeout = Duration(milliseconds: 500);

class DiscoveryService {
  RawDatagramSocket? _oldSocket;

  DiscoveryService() {
    if (kDebugMode) {
      serviceLocator.get<DiscoveryBloc>().add(DiscoveredEvent(
          const String.fromEnvironment('RUSTY_DEBUG_SERVER_IP',
              defaultValue: '127.0.0.1')));
    } else {
      serviceLocator.get<DiscoveryBloc>().add(NotConnectedEvent());
    }
  }

  void discover() async {
    _oldSocket?.close();
    final socket = await _bind();
    final broadcastAddress = await NetworkInfo().getWifiBroadcast();

    if (broadcastAddress == null) {
      log.i('No Wifi broadcast address found');
      socket.close();
      return;
    }

    _oldSocket = socket;

    // Substr because this starts with '/' ("/192.168.150.255")
    final address = broadcastAddress.substring(1).convertAddressToBytes();
    final request = utf8.encode(handshakeResponse);

    socket.send(
        Uint8List.fromList(request),
        InternetAddress.fromRawAddress(Uint8List.fromList(address)),
        handshakeBeginPort);
  }

  Future<RawDatagramSocket> _bind() async {
    final socket =
        await RawDatagramSocket.bind(InternetAddress.anyIPv4, handshakeEndPort);
    socket.broadcastEnabled = true;

    var handshakeCompleted = false;
    socket.listen(
        (_) => _onPacketReceived(socket, () => handshakeCompleted = true));

    Future.delayed(handshakeTimeout).then((_) {
      if (!handshakeCompleted) {
        serviceLocator.get<DiscoveryBloc>().add(NotConnectedEvent());
      }
    });

    return socket;
  }

  void _onPacketReceived(RawDatagramSocket socket, Function() onHandshake) {
    final packet = socket.receive();

    if (packet == null) {
      log.v('Received nothing from UDP socket');
      return;
    }

    final response = utf8.decode(packet.data);

    if (response != handshakeRequest) {
      log.d('Received invalid response from socket. (response = $response)');
      return;
    }

    final ip = packet.address.address;

    log.i('Received handshake response from $ip!');

    serviceLocator.get<DiscoveryBloc>().add(DiscoveredEvent(ip));
    onHandshake();
  }
}
