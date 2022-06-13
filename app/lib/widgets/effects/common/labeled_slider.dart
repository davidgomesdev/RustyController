import 'package:flutter/material.dart';

class LabeledSlider extends StatelessWidget {
  final double value, min, max;
  final String label;
  final void Function(double) onChanged;

  const LabeledSlider(
      {Key? key,
      required this.onChanged,
      required this.label,
      required this.value,
      this.max = 1.0,
      this.min = 0.0})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Text(label),
        Slider(
          value: value,
          max: max,
          min: min,
          onChanged: onChanged,
        ),
      ],
    );
  }
}
