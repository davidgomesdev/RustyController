import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/bloc/effects/rainbow_bloc.dart';
import 'package:rusty_controller/global_consts.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_settings/common/labeled_slider.dart';

class RainbowSettings extends StatefulWidget {
  const RainbowSettings({super.key});

  @override
  State<RainbowSettings> createState() => _RainbowSettingsState();
}

class _RainbowSettingsState extends State<RainbowSettings> {
  final bloc = serviceLocator.get<RainbowBloc>();

  @override
  Widget build(BuildContext context) {
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
                setState(() => bloc.add(RainbowTimeEvent(time)));
              },
            ),
            Row(
              children: [
                Flexible(
                  child: LabeledSlider(
                    label: 'Saturation',
                    value: effect.saturation,
                    onChanged: (saturation) {
                      setState(
                          () => bloc.add(RainbowSaturationEvent(saturation)));
                    },
                  ),
                ),
                Flexible(
                  child: LabeledSlider(
                    label: 'Brightness',
                    value: effect.value,
                    onChanged: (value) {
                      setState(() => bloc.add(RainbowValueEvent(value)));
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
