import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_colorpicker/flutter_colorpicker.dart';
import 'package:rusty_controller/bloc/effects/candle_bloc.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_settings/common/labeled_slider.dart';

class CandleSettings extends StatelessWidget {
  final initialColor = const HSVColor.fromAHSV(1.0, 0.0, 1.0, 1.0);

  const CandleSettings({super.key});

  @override
  Widget build(BuildContext context) {
    final bloc = serviceLocator.get<CandleBloc>();
    return BlocBuilder<CandleBloc, CandleLedEffect>(
      bloc: bloc,
      builder: (ctx, effect) {
        final color = initialColor.withHue(effect.hue);

        return Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Flexible(
              child: Padding(
                padding: EdgeInsets.only(bottom: 48.0),
                child: Stack(
                  alignment: Alignment.center,
                  children: [
                    ColorIndicator(color, width: 60.0, height: 60.0),
                    SizedBox(
                      width: 200,
                      height: 200,
                      child: ColorPickerHueRing(
                        color,
                        displayThumbColor: false,
                        strokeWidth: 30.0,
                        (color) {
                          bloc.add(CandleEffectEvent(hue: color.hue));
                        },
                      ),
                    )
                  ],
                ),
              ),
            ),
            Column(
              children: [
                LabeledRangeSlider(
                  onChanged: (min, max) {
                    bloc.add(CandleEffectEvent(minValue: min, maxValue: max));
                  },
                  label: "Brightness range",
                  start: effect.minValue,
                  end: effect.maxValue,
                ),
                LabeledSlider(
                  onChanged: (interval) {
                    bloc.add(CandleEffectEvent(interval: interval.toInt()));
                  },
                  label: "Interval",
                  value: effect.interval.toDouble(),
                  min: 100,
                  max: 800,
                ),
              ],
            ),
          ],
        );
      },
    );
  }
}
