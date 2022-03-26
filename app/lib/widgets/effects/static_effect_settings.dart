import 'dart:async';

import 'package:flutter/material.dart';
import 'package:rusty_controller/bloc/events/led_effects.dart';
import 'package:rusty_controller/widgets/settings_widgets.dart';

class StaticEffectSettings extends StatelessWidget {
  final StreamSink<LedEffectEvent> effectStream;

  final StreamController<Color> _colorStream = StreamController();
  final StaticLedEffectEvent _currentEffect;

  StaticEffectSettings({Key? key, required this.effectStream})
      : _currentEffect = StaticLedEffectEvent(color: Colors.black),
        super(key: key);

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<Color>(
        stream: _colorStream.stream,
        initialData: Colors.black,
        builder: (context, snapshot) {
          final selectedColor = snapshot.data;

          if (selectedColor != null) {
            effectStream.add(_currentEffect..color = selectedColor);
          }

          return LedColorPicker(
            colorPickStream: _colorStream.sink,
          );
        });
  }
}
