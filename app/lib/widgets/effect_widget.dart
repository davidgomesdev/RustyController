import 'package:flutter/material.dart';
import 'package:rusty_controller/bloc/effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effects/breathing_settings.dart';
import 'package:rusty_controller/widgets/effects/off_settings.dart';
import 'package:rusty_controller/widgets/effects/static_settings.dart';

class EffectWidget<T extends LedEffect> extends StatelessWidget {
  final T currentEffect;
  final EffectBloc bloc;

  const EffectWidget(this.currentEffect, {Key? key, required this.bloc})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    final effect = currentEffect;

    if (effect is StaticEffect) {
      return StaticEffectWidget(effectBloc: bloc, currentEffect: effect);
    } else if (effect is BreathingEffect) {
      return BreathingSettings(effectBloc: bloc, effect: effect);
    } else if (effect is OffEffect) {
      return const OffEffectWidget();
    } else {
      return Container();
    }
  }
}
