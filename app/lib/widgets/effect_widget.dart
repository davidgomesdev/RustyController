import 'dart:async';

import 'package:flutter/material.dart';
import 'package:rusty_controller/bloc/events/led_effects.dart';
import 'package:rusty_controller/widgets/effects/breathing_effect.dart';
import 'package:rusty_controller/widgets/effects/off_effect.dart';
import 'package:rusty_controller/widgets/effects/static_effect.dart';

class EffectWidget<T extends EffectEvent> extends StatelessWidget {
  final T currentEffect;
  final StreamSink<EffectEvent> colorStream;

  const EffectWidget(this.currentEffect, {Key? key, required this.colorStream})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    final effect = currentEffect;

    if (effect is StaticEffectEvent) {
      return StaticEffect(
        effectStream: colorStream,
        currentEffect: effect,
      );
    } else if (effect is BreathingEffectEvent) {
      return BreathingSettings(
          effectStream: colorStream, currentEffect: effect);
    } else if (effect is OffEffectEvent) {
      return const OffEffect();
    } else {
      return Container();
    }
  }
}
