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
    return BlocBuilder<BreathingBloc, BreathingEffect>(
      bloc: bloc,
      builder: (ctx, effect) => Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          LedColorPicker(
            currentColor: effect.color,
            onColorPick: (color) {
              setState(() {
                if (color.value > effect.peak) effect.peak = color.value;

                bloc.add(BreathingColorEvent(color));
              });
            },
          ),
          Column(
            children: [
              LabeledSlider(
                label: 'Step',
                value: effect.step,
                min: minBreathingStep,
                max: maxBreathingStep,
                onChanged: (step) {
                  setState(() {
                    bloc.add(BreathingStepEvent(step));
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
      ),
    );
  }
}
