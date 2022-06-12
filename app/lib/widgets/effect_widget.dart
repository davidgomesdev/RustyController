import 'package:flutter/material.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effects/breathing_settings.dart';
import 'package:rusty_controller/widgets/effects/off_settings.dart';
import 'package:rusty_controller/widgets/effects/static_settings.dart';

class EffectWidget<T extends LedEffect> extends StatelessWidget {
  final T currentEffect;

  const EffectWidget(this.currentEffect, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final effect = currentEffect;

    if (effect is StaticEffect) {
      return StaticEffectWidget(currentEffect: effect);
    } else if (effect is BreathingEffect) {
      return BreathingSettings(effect: effect);
    } else if (effect is OffEffect) {
      return const OffEffectWidget();
    } else {
      return Container();
    }
  }
}
