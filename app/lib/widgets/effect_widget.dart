import 'dart:async';

import 'package:flutter/material.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effects/breathing_effect.dart';
import 'package:rusty_controller/widgets/effects/off_effect.dart';
import 'package:rusty_controller/widgets/effects/static_effect.dart';

class EffectWidget<T extends LedEffect> extends StatelessWidget {
  final T currentEffect;
  final StreamSink<LedEffect> colorStream;

  const EffectWidget(this.currentEffect, {Key? key, required this.colorStream})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    final effect = currentEffect;

    if (effect is StaticEffect) {
      return StaticEffectWidget(
        effectStream: colorStream,
        currentEffect: effect,
      );
    } else if (effect is BreathingEffect) {
      return BreathingSettings(effectStream: colorStream, effect: effect);
    } else if (effect is OffEffect) {
      return const OffEffectWidget();
    } else {
      return Container();
    }
  }
}
