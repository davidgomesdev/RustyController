import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:rusty_controller/bloc/discovery_bloc.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/graphql_queries.dart';
import 'package:rusty_controller/model/led_effects.dart';

class ControllerService {
  late GraphQLClient _graphqlClient;

  final Map<EffectType, LedEffect> _effects = {
    EffectType.off: OffEffect(),
    EffectType.static: StaticEffect(color: Colors.black.toHSV()),
    EffectType.breathing:
        BreathingEffect(color: Colors.black.toHSV(), step: 0.01, peak: 1.0),
    EffectType.rainbow: RainbowEffect(saturation: 1.0, value: 1.0, step: 1),
  };

  void connect(String ip) {
    _graphqlClient = GraphQLClient(
      link: HttpLink("http://$ip:8080/graphql"),
      cache: GraphQLCache(store: InMemoryStore()),
    );

    _graphqlClient.query(QueryOptions(document: gql(healthQuery))).then((_) {
      serviceLocator.get<DiscoveryBloc>().add(ConnectedEvent());
    })._reconnectOnTimeout();
  }

  Future<void> _sendEffect(LedEffect effect) async {
    log.d("Sending mutation for '${effect.name}' effect");
    await _graphqlClient
        .mutate(MutationOptions(document: gql(effect.graphqlMutation)))
        .then(log.v, onError: (msg, _) => log.e(msg))
        ._reconnectOnTimeout();
  }

  void set({required LedEffect effect}) {
    log.i("Setting effect to '${effect.name}'");

    _effects[effect.type] = effect;

    _sendEffect(effect);
  }

  LedEffect get({required EffectType type}) {
    final effect = _effects[type];

    if (effect == null) throw ArgumentError.notNull("effect");

    return effect;
  }
}

extension on Future {
  Future _reconnectOnTimeout() {
    return timeout(const Duration(seconds: 3), onTimeout: () {
      serviceLocator.get<DiscoveryBloc>().add(NotConnectedEvent());
    });
  }
}
