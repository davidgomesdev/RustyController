import 'package:flutter/cupertino.dart';
import 'package:json_annotation/json_annotation.dart';

class HSVColorJsonConverter
    extends JsonConverter<HSVColor, Map<String, dynamic>> {
  const HSVColorJsonConverter();

  @override
  HSVColor fromJson(Map<String, dynamic> json) {
    return HSVColor.fromAHSV(
        json['alpha'], json['hue'], json['saturation'], json['value']);
  }

  @override
  Map<String, dynamic> toJson(HSVColor object) {
    return {
      "alpha": object.alpha,
      "hue": object.hue,
      "saturation": object.saturation,
      "value": object.value,
    };
  }
}
