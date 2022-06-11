import 'package:flutter/material.dart';
import 'package:flutter_colorpicker/flutter_colorpicker.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';

class LedColorPicker extends StatelessWidget {
  final HSVColor currentColor;
  final ValueSetter<HSVColor> onColorPick;

  const LedColorPicker(
      {Key? key, required this.currentColor, required this.onColorPick})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: SlidePicker(
        pickerColor: currentColor.toColor(),
        onColorChanged: (color) => onColorPick(color.toHSV()),
        colorModel: ColorModel.hsv,
        showParams: false,
        showSliderText: false,
        enableAlpha: false,
      ),
    );
  }
}
