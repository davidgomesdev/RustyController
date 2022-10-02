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

    if (effect is StaticLedEffect) {
      return const StaticSettings();
    } else if (effect is BreathingLedEffect) {
      return const BreathingSettings();
    } else if (effect is RainbowLedEffect) {
      return const RainbowSettings();
    } else if (effect is OffLedEffect) {
      return const OffEffectWidget();
    } else {
      return Container();
    }
  }
}
