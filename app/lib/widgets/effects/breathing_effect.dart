import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:rusty_controller/bloc/events/led_effects.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';
import 'package:rusty_controller/widgets/effects/static_effect.dart';
import 'package:rusty_controller/widgets/effects/common/led_color_picker.dart';

class BreathingSettings extends StatefulWidget {
  final StreamSink<EffectEvent> effectStream;
  final StreamController<HSVColor> _colorStream = StreamController<HSVColor>();

  final BreathingEffectEvent currentEffect;

  BreathingSettings(
      {Key? key, required this.effectStream, required this.currentEffect})
      : super(key: key);

  @override
  State<BreathingSettings> createState() => _BreathingSettingsState();
}

class _BreathingSettingsState extends State<BreathingSettings> {
  HSVColor _currentColor = Colors.black.toHSV();
  double _step = 0.0;
  double _peak = 0.0;

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<HSVColor>(
        stream: widget._colorStream.stream,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.active &&
              snapshot.hasData) {
            _currentColor = snapshot.data!;
            widget.currentEffect
              ..color = _currentColor
              ..step = _step
              ..peak = _peak;

            widget.effectStream.add(widget.currentEffect);
          }

          return Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              LedColorPicker(
                currentColor: _currentColor,
                colorPickStream: widget._colorStream.sink,
              ),
              Row(
                children: [
                  Flexible(
                    child: Slider(
                      value: _step,
                      label: "Step",
                      onChanged: (value) {
                        setState(() {
                          _step = value;
                        });
                      },
                    ),
                  ),
                  Flexible(
                    child: Slider(
                      value: _peak,
                      label: "Peak",
                      onChanged: (value) {
                        setState(() {
                          _peak = value;
                        });
                      },
                    ),
                  ),
                ],
              )
            ],
          );
        });
  }
}
