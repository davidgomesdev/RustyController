import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/bloc/effects/breathing_bloc.dart';
import 'package:rusty_controller/global_consts.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_settings/common/labeled_slider.dart';
import 'package:rusty_controller/widgets/effect_settings/common/led_color_picker.dart';

class BreathingSettings extends StatefulWidget {
  const BreathingSettings({Key? key}) : super(key: key);

  @override
  State<BreathingSettings> createState() => _BreathingSettingsState();
}

class _BreathingSettingsState extends State<BreathingSettings> {
  final bloc = serviceLocator.get<BreathingBloc>();

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BreathingBloc, BreathingLedEffect>(
      bloc: bloc,
      builder: (ctx, effect) {
        return Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            LedColorPicker(
              currentColor: effect.color,
              ignoreValue: effect.breatheFromOff,
              onColorPick: (color) {
                if (color.value > effect.peak) effect.peak = color.value;

                if (effect.breatheFromOff) {
                  setState(() {
                    bloc.add(BreathingColorEvent(color));
                });
              } else {
                // can't `setState`, otherwise the color conversion will
                // prevent the hue and saturation from sliding
                // when the value one is at 0.0
                bloc.add(BreathingColorEvent(color));
              }
            },
          ),
          Column(
            children: [
              SwitchListTile.adaptive(
                  value: effect.breatheFromOff,
                  onChanged: (fromOff) {
                    setState(() {
                      bloc.add(BreathingFromOffEvent(fromOff));
                    });
                  },
                  title: const Text("Breathe from off")),
              LabeledLogSlider(
                  label: 'Step',
                  value: effect.step.toDouble(),
                  min: minBreathingStep.toDouble(),
                  max: maxBreathingStep.toDouble(),
                  onChanged: (step) {
                    setState(() {
                      bloc.add(BreathingStepEvent(step.round()));
                    });
                  },
                ),
              LabeledSlider(
                label: 'Peak',
                value: effect.peak,
                onChanged: (peak) {
                  if (peak < effect.color.value) {
                    peak = effect.color.value;
                    }

                    setState(() {
                      bloc.add(BreathingPeakEvent(peak));
                    });
                  },
                ),
              ],
            )
          ],
        );
      },
    );
  }
}
