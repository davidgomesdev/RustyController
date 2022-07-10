import 'package:flutter/material.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_settings/breathing_settings.dart';
import 'package:rusty_controller/widgets/effect_settings/off_settings.dart';
import 'package:rusty_controller/widgets/effect_settings/rainbow_settings.dart';
import 'package:rusty_controller/widgets/effect_settings/static_settings.dart';

class EffectWidget<T extends LedEffect> extends StatelessWidget {
  final T currentEffect;

  const EffectWidget(this.currentEffect, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final effect = currentEffect;

    if (effect is StaticEffect) {
      return const StaticSettings();
    } else if (effect is BreathingEffect) {
      return const BreathingSettings();
    } else if (effect is RainbowEffect) {
      return const RainbowSettings();
    } else if (effect is OffEffect) {
      return const OffEffectWidget();
    } else {
      return Container();
    }
  }
}
