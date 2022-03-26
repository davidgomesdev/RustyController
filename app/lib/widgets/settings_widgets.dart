import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_colorpicker/flutter_colorpicker.dart';

class LedColorPicker extends StatelessWidget {
  final StreamSink<Color> colorPickStream;

  const LedColorPicker({Key? key, required this.colorPickStream})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: SlidePicker(
        pickerColor: Colors.black,
        onColorChanged: (color) => colorPickStream.add(color),
        colorModel: ColorModel.hsv,
        showParams: false,
        showSliderText: false,
        enableAlpha: false,
      ),
    );
  }
}
