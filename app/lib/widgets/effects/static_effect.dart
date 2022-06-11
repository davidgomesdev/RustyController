import 'dart:async';

import 'package:flutter/material.dart';
import 'package:rusty_controller/bloc/events/led_effects.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';
import 'package:rusty_controller/widgets/effects/common/led_color_picker.dart';

class StaticEffect extends StatelessWidget {

  final StreamSink<EffectEvent> effectStream;
  final StaticEffectEvent currentEffect;

  final StreamController<HSVColor> _colorStream = StreamController();

  StaticEffect({Key? key, required this.effectStream, required this.currentEffect})
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
            colorPickStream: _colorStream.sink,
          );
        });
  }
}
