import 'dart:async';

import 'package:flutter/material.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effects/common/led_color_picker.dart';

class StaticEffectWidget extends StatelessWidget {
  final StreamSink<LedEffect> effectStream;
  final StaticEffect currentEffect;

  final StreamController<HSVColor> _colorStream = StreamController();

  StaticEffectWidget(
      {Key? key, required this.effectStream, required this.currentEffect})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<HSVColor>(
        stream: _colorStream.stream,
        initialData: currentEffect.color,
        builder: (context, snapshot) {
          final selectedColor = snapshot.data ?? currentEffect.color;

          if (snapshot.connectionState == ConnectionState.active) {
            effectStream.add(currentEffect..color = selectedColor);
          }

          return LedColorPicker(
            currentColor: selectedColor,
            onColorPick: _colorStream.sink.add,
          );
        });
  }
}
