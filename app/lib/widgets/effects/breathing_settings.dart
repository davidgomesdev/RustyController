import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/bloc/effects/breathing_bloc.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effects/common/led_color_picker.dart';

class BreathingSettings extends StatefulWidget {
  final BreathingEffect effect;

  const BreathingSettings({Key? key, required this.effect}) : super(key: key);

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
          Row(
            children: [
              Flexible(
                    child: Slider(
                      value: effect.step,
                  max: 0.01,
                  onChanged: (step) {
                    setState(() {
                      bloc.add(BreathingStepEvent(step));
                    });
                  },
                ),
                  ),
                  Flexible(
                    child: Slider(
                      value: effect.peak,
                      label: "Peak",
                      onChanged: (peak) {
                        if (peak > effect.color.value) {
                          setState(() {
                            bloc.add(BreathingPeakEvent(peak));
                          });
                        }
                      },
                    ),
                  ),
                ],
              )
            ],
          ),
    );
  }
}
