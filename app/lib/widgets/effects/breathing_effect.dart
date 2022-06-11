import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/bloc/breathing_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effects/common/led_color_picker.dart';

class BreathingSettings extends StatefulWidget {
  final StreamSink<LedEffect> effectStream;

  late final BreathingBloc _bloc;

  BreathingSettings(
      {Key? key, required this.effectStream, required BreathingEffect effect})
      : super(key: key) {
    _bloc = BreathingBloc(effect);
  }

  @override
  State<BreathingSettings> createState() => _BreathingSettingsState();
}

class _BreathingSettingsState extends State<BreathingSettings> {
  @override
  Widget build(BuildContext context) {
    return BlocConsumer<BreathingBloc, BreathingEffect>(
      bloc: widget._bloc,
      listener: (ctx, effect) {
        widget.effectStream.add(effect);
      },
      builder: (ctx, effect) => Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          LedColorPicker(
            currentColor: effect.color,
            onColorPick: (color) =>
                widget._bloc.add(BreathingColorEvent(color)),
          ),
          Row(
            children: [
              Flexible(
                child: Slider(
                  value: effect.step,
                  label: "Step",
                  onChanged: (step) {
                    setState(() {
                      widget._bloc.add(BreathingStepEvent(step));
                    });
                  },
                ),
              ),
              Flexible(
                child: Slider(
                  value: effect.peak,
                  label: "Peak",
                  onChanged: (peak) {
                    setState(() {
                      if (peak > effect.color.value) {
                        widget._bloc.add(BreathingPeakEvent(peak));
                      }
                    });
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
