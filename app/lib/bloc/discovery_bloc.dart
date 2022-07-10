import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/service/controller_service.dart';
import 'package:rusty_controller/service/discovery_service.dart';

class DiscoveryBloc extends Bloc<ConnectionEvent, RustyConnectionState> {
  DiscoveryBloc() : super(DisconnectedState()) {
    on<NotConnectedEvent>((_, emit) async {
      serviceLocator.get<DiscoveryService>().discover();
    });
    on<DiscoveredEvent>((event, _) async {
      serviceLocator.get<ControllerService>().connect(event.ip);
    });
    on<ConnectedEvent>((_, emit) async {
      emit(ConnectedState());
    });
  }
}

abstract class ConnectionEvent {}

class NotConnectedEvent extends ConnectionEvent {}

/// The IP of Rusty was found, but we're not yet connected
///
/// Used on the Bloc to get the service to connect to its found IP
///
/// Does not emit, since we're not yet connected
class DiscoveredEvent extends ConnectionEvent {
  final String ip;

  DiscoveredEvent(this.ip);
}

class ConnectedEvent extends ConnectionEvent {}

abstract class RustyConnectionState {}

class DisconnectedState extends RustyConnectionState {}

class ConnectedState extends RustyConnectionState {}
