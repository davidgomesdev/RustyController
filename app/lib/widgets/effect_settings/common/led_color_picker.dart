import 'package:flutter/material.dart';
import 'package:flutter_colorpicker/flutter_colorpicker.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';

class LedColorPicker extends StatelessWidget {
  final HSVColor currentColor;
  final bool ignoreValue;
  final void Function(HSVColor) onColorPick;

  const LedColorPicker(
      {Key? key,
      required this.currentColor,
      required this.onColorPick,
      this.ignoreValue = false})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    var hsvColor = currentColor;

    if (ignoreValue) hsvColor = hsvColor.withValue(1.0);

    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: SlidePicker(
        pickerColor: hsvColor.toColor(),
        onColorChanged: (color) {
          var hsvColor = color.toHSV();

          if (ignoreValue) hsvColor = hsvColor.withValue(0.0);

          onColorPick(hsvColor);
        },
        colorModel: ColorModel.hsv,
        showParams: false,
        showSliderText: false,
        enableAlpha: false,
      ),
    );
  }
}
