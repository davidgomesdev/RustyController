import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:get/get.dart';
import 'package:rusty_controller/bloc/effects/breathing_bloc.dart';
import 'package:rusty_controller/global_consts.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_settings/common/labeled_slider.dart';
import 'package:rusty_controller/widgets/effect_settings/common/led_color_picker.dart';

class BreathingSettings extends StatefulWidget {
  const BreathingSettings({super.key});

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
        return Column(mainAxisAlignment: MainAxisAlignment.center, children: [
          LedColorPicker(
            currentColor: effect.color,
            ignoreValue: effect.breatheFromOff,
            onColorPick: (color) {
              setState(() {
                if (!effect.breatheFromOff && isBrightnessOff(effect, color)) {
                  Get.closeAllSnackbars();
                  Get.rawSnackbar(
                    message: 'You need to increase the brightness!',
                  );
                } else {
                  bloc.add(BreathingColorEvent(color));

                  if (color.value > effect.peak) {
                    effect.peak = color.value;
                  }
                }
              });
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
                label: 'Time to peak',
                value: effect.timeToPeak.toDouble(),
                min: minBreathingTime.toDouble(),
                max: maxBreathingTime.toDouble(),
                onChanged: (time) {
                  setState(() {
                    bloc.add(BreathingTimeEvent(time.round()));
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
        ]);
      },
    );
  }

  bool isBrightnessOff(BreathingLedEffect effect, HSVColor color) {
    return effect.color.value == 0 && color.value == 0;
  }
}
