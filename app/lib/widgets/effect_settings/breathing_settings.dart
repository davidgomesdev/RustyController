import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:get/get.dart';
import 'package:rusty_controller/bloc/effects/breathing_bloc.dart';
import 'package:rusty_controller/global_consts.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_settings/common/labeled_slider.dart';
import 'package:rusty_controller/widgets/effect_settings/common/led_color_picker.dart';

class BreathingSettings extends StatelessWidget {
  const BreathingSettings({super.key});

  @override
  Widget build(BuildContext context) {
    final bloc = serviceLocator.get<BreathingBloc>();

    return BlocBuilder<BreathingBloc, BreathingLedEffect>(
      bloc: bloc,
      builder: (ctx, effect) {
        return Column(mainAxisAlignment: MainAxisAlignment.center, children: [
          LedColorPicker(
            currentColor: effect.color,
            ignoreValue: effect.breatheFromOff,
            onColorPick: (color) {
              if (!effect.breatheFromOff && isBrightnessOff(effect, color)) {
                Get.closeAllSnackbars();
                Get.rawSnackbar(
                  message: 'You need to increase the brightness!',
                );
              } else {
                bloc.add(BreathingEffectEvent(color: color));
              }
            },
          ),
          Column(
            children: [
              SwitchListTile.adaptive(
                  value: effect.breatheFromOff,
                  onChanged: (fromOff) {
                    bloc.add(BreathingEffectEvent(breatheFromOff: fromOff));
                  },
                  title: const Text("Breathe from off")),
              LabeledLogSlider(
                label: 'Time to peak',
                value: effect.timeToPeak.toDouble(),
                min: minBreathingTime.toDouble(),
                max: maxBreathingTime.toDouble(),
                onChanged: (time) {
                  bloc.add(BreathingEffectEvent(timeToPeak: time.round()));
                },
              ),
              LabeledSlider(
                label: 'Peak',
                value: effect.peak,
                onChanged: (peak) {
                  if (peak < effect.color.value) {
                    peak = effect.color.value;
                  }

                  bloc.add(BreathingEffectEvent(peak: peak));
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
