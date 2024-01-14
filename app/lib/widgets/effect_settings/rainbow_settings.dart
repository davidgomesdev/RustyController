import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/bloc/effects/rainbow_bloc.dart';
import 'package:rusty_controller/global_consts.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_settings/common/labeled_slider.dart';

class RainbowSettings extends StatelessWidget {
  const RainbowSettings({super.key});

  @override
  Widget build(BuildContext context) {
    final bloc = serviceLocator.get<RainbowBloc>();

    return BlocBuilder<RainbowBloc, RainbowLedEffect>(
      bloc: bloc,
      builder: (ctx, effect) {
        return Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            LabeledLogSlider(
              label: 'Time to complete',
              value: effect.timeToComplete,
              min: minRainbowTime,
              max: maxRainbowTime,
              onChanged: (time) {
                bloc.add(RainbowEffectEvent(timeToComplete: time));
              },
            ),
            Row(
              children: [
                Flexible(
                  child: LabeledSlider(
                    label: 'Saturation',
                    value: effect.saturation,
                    onChanged: (saturation) {
                      bloc.add(RainbowEffectEvent(saturation: saturation));
                    },
                  ),
                ),
                Flexible(
                  child: LabeledSlider(
                    label: 'Brightness',
                    value: effect.value,
                    onChanged: (value) {
                      bloc.add(RainbowEffectEvent(value: value));
                    },
                  ),
                ),
              ],
            )
          ],
        );
      },
    );
  }
}
