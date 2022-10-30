import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:rusty_controller/bloc/discovery_bloc.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';
import 'package:rusty_controller/global_consts.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/graphql_queries.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/service/store_service.dart';

class ControllerService {
  late GraphQLClient _graphqlClient;

  final Map<EffectType, LedEffect> _effects = {
    EffectType.off: OffLedEffect(),
    EffectType.static: StaticLedEffect(color: Colors.black.toHSV()),
    EffectType.breathing: BreathingLedEffect(
        color: Colors.red.toHSV().withValue(0.0),
        step: maxBreathingStep,
        peak: 1.0,
        breatheFromOff: true),
    EffectType.rainbow:
        RainbowLedEffect(saturation: 1.0, value: 0.5, step: maxRainbowStep),
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

  void set({required LedEffect effect}) {
    log.i("Setting effect to '${effect.name}'");

    _effects[effect.type] = effect;

    _sendEffect(effect);
    _saveEffect(effect);
  }

  LedEffect get({required EffectType type}) {
    final effect = _effects[type];

    if (effect == null) throw ArgumentError.notNull("effect");

    return effect;
  }

  Future<void> _sendEffect(LedEffect effect) async {
    log.i("Sending mutation for '${effect.name}' effect");
    log.d(
        "Mutation input: ${effect.graphqlVariables} for '${effect.graphqlMutationName}'");

    await _graphqlClient
        .mutate(MutationOptions(
            document: gql(effect.graphqlMutation),
            variables: {"input": effect.graphqlVariables}))
        .then((msg) {
      if (msg.hasException) {
        final exception = msg.exception!.linkException;
        if (exception is NetworkException || exception is ServerException) {
          log.w('Network error when sending effect', exception);
          serviceLocator.get<DiscoveryBloc>().add(NotConnectedEvent());
          return;
        }

        log.e(msg.exception);
        return;
      }

      if (msg.data?[effect.graphqlMutationName] == "SUCCESS") {
        log.i("Mutation succeeded");
      } else {
        log.w("Server didn't respond successfully to mutation.", msg.data);
      }
    }, onError: (msg, _) => log.e(msg))._reconnectOnTimeout();
  }

  Future<void> _saveEffect(LedEffect effect) async {
    if (effect is StorableObject) {
      serviceLocator.get<StoreService>().save(effect as StorableObject);
    }
  }
}

extension on Future {
  Future _reconnectOnTimeout() {
    return timeout(const Duration(seconds: 3), onTimeout: () {
      serviceLocator.get<DiscoveryBloc>().add(NotConnectedEvent());
    });
  }
}
