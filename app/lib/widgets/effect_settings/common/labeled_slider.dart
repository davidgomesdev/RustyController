import 'dart:math' as math;

import 'package:flutter/material.dart';

class LabeledLogSlider extends StatelessWidget {
  final double value, min, max;
  final double scale;
  final String label;
  final void Function(double) onChanged;

  LabeledLogSlider(
      {super.key,
      required this.onChanged,
      required this.label,
      required this.value,
      this.min = 1.0,
      this.max = 10.0})
      : scale = log(max) - log(min);

  @override
  Widget build(BuildContext context) {
    return LabeledSlider(
        onChanged: (double position) => onChanged(getLogValue(position)),
        label: label,
        value: getPosition(value));
  }

  double getLogValue(double slidePosition) {
    return math.exp(scale * slidePosition + log(min));
  }

  double getPosition(double position) {
    return (log(position) - log(min)) / scale;
  }

  static double log(double num) {
    if (num == 0.0) return 0.0;

    return math.log(num);
  }
}

class LabeledSlider extends StatelessWidget {
  final double value, min, max;
  final String label;
  final void Function(double) onChanged;

  const LabeledSlider(
      {super.key,
      required this.onChanged,
      required this.label,
      required this.value,
      this.max = 1.0,
      this.min = 0.0});

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
