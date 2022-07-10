import 'dart:isolate';

import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/service/worker_service.dart';

final recv = ReceivePort();

class ControllerService {
// TODO: use UDP discovery
  final _graphqlClient = GraphQLClient(
    link: HttpLink("http://127.0.0.1:8080/graphql"),
    cache: GraphQLCache(store: InMemoryStore()),
  );

  final Map<EffectType, LedEffect> _effects = {
    EffectType.off: OffEffect(),
    EffectType.static: StaticEffect(color: Colors.black.toHSV()),
    EffectType.breathing:
        BreathingEffect(color: Colors.black.toHSV(), step: 0.01, peak: 1.0),
    EffectType.rainbow: RainbowEffect(saturation: 1.0, value: 1.0, step: 1),
  };

  ControllerService() {
    serviceLocator.registerSingletonAsync(() async =>
        await WorkerService.create<LedEffect>((effect) async {
          log.d("Sending mutation for '${effect.name}' effect");
          await _graphqlClient
              .mutate(MutationOptions(document: gql(effect.graphqlMutation)))
              .then(log.v, onError: (msg, _) => log.e(msg));
        }));
  }

  void set({required LedEffect effect}) {
    log.i("Setting effect to '${effect.name}'");

    _effects[effect.type] = effect;

    serviceLocator.get<WorkerService<LedEffect>>().send(effect);
  }

  LedEffect get({required EffectType type}) {
    final effect = _effects[type];

    if (effect == null) throw ArgumentError.notNull("effect");

    return effect;
  }
}