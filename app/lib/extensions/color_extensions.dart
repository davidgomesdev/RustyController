import 'package:flutter/painting.dart';

extension HSVConversion on Color {
  HSVColor toHSV() {
    return HSVColor.fromColor(this);
  }
}