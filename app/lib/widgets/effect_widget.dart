import 'package:flutter/material.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_settings/breathing_settings.dart';
import 'package:rusty_controller/widgets/effect_settings/candle_settings.dart';
import 'package:rusty_controller/widgets/effect_settings/off_settings.dart';
import 'package:rusty_controller/widgets/effect_settings/rainbow_settings.dart';
import 'package:rusty_controller/widgets/effect_settings/static_settings.dart';

class EffectWidget<T extends LedEffect> extends StatelessWidget {
  final T currentEffect;

  const EffectWidget(this.currentEffect, {super.key});

  @override
  Widget build(BuildContext context) {
    final effect = currentEffect;

    if (effect is StaticLedEffect) {
      return const StaticSettings();
    } else if (effect is BreathingLedEffect) {
      return const BreathingSettings();
    } else if (effect is CandleLedEffect) {
      return const CandleSettings();
    } else if (effect is RainbowLedEffect) {
      return const RainbowSettings();
    } else if (effect is OffLedEffect) {
      return const OffEffectWidget();
    } else if (effect is NoLedEffect) {
      return Container();
    } else {
      throw ArgumentError("No widget for effect $effect");
    }
  }
}
