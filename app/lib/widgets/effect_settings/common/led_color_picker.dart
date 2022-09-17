import 'package:flutter/material.dart';
import 'package:flutter_colorpicker/flutter_colorpicker.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';

class LedColorPicker extends StatefulWidget {
  final HSVColor currentColor;
  final bool ignoreValue;
  final void Function(HSVColor) onColorPick;

  /// [ignoreValue] ignores [currentColor.value], overriding to 1.0
  const LedColorPicker(
      {Key? key,
      required this.currentColor,
      required this.onColorPick,
      this.ignoreValue = false})
      : super(key: key);

  @override
  State<LedColorPicker> createState() => _LedColorPickerState();
}

class _LedColorPickerState extends State<LedColorPicker> {
  @override
  Widget build(BuildContext context) {
    var hsvColor = widget.currentColor;

    if (widget.ignoreValue) hsvColor = hsvColor.withValue(1.0);

    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: SlidePicker(
        pickerColor: hsvColor.toColor(),
        onColorChanged: (color) {
          var changedHsvColor = color.toHSV();

          if (widget.ignoreValue && changedHsvColor.value != hsvColor.value) {
            // to prevent changing the value slider
            setState(() {});
            return;
          }

          if (widget.ignoreValue) {
            changedHsvColor = changedHsvColor.withValue(0.0);
          }

          widget.onColorPick(changedHsvColor);
        },
        colorModel: ColorModel.hsv,
        showParams: false,
        showSliderText: false,
        enableAlpha: false,
      ),
    );
  }
}
