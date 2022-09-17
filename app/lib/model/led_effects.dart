import 'package:flutter/painting.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:rusty_controller/model/util/json_converters.dart';
import 'package:rusty_controller/service/store_service.dart';

part 'led_effects.g.dart';

abstract class LedEffect {
  EffectType get type;

  String get name => type.name;

  String get graphqlMutation;

  StorableObject? get storeObject => null;
}

enum EffectType { none, off, static, breathing, rainbow }

// A placeholder/null-object pattern for when there's no effect selected,
// while avoiding null-checks
class NoEffect extends LedEffect {
  @override
  EffectType get type => EffectType.none;

  @override
  String get graphqlMutation => "";
}

class OffEffect extends LedEffect {
  @override
  EffectType get type => EffectType.off;

  @override
  String get graphqlMutation => """
    mutation SetLedOff {
      off
    }
  """;
}

@JsonSerializable()
@HSVColorJsonConverter()
class StaticEffect extends LedEffect implements StorableObject {
  @override
  EffectType get type => EffectType.static;

  HSVColor color;

  StaticEffect({required this.color});

  @override
  String get graphqlMutation => """
    mutation SetLedStatic {
      static(h: ${color.hue}, s: ${color.saturation}, v: ${color.value})
    }
  """;

  @override
  String get storeName => "static";

  @override
  StaticEffect fromJson(Map<String, dynamic> json) =>
      _$StaticEffectFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$StaticEffectToJson(this);
}

@JsonSerializable()
@HSVColorJsonConverter()
class BreathingEffect extends LedEffect implements StorableObject {
  @override
  EffectType get type => EffectType.breathing;

  HSVColor color;
  double step;
  double peak;
  bool breatheFromOff;

  BreathingEffect(
      {required this.color,
      required this.step,
      required this.peak,
      required this.breatheFromOff});

  @override
  String get graphqlMutation => """
    mutation SetLedBreathing {
      breathing(h: ${color.hue}, s: ${color.saturation}, initialV: ${color.value}, step: $step, peak: $peak)
    }
  """;

  @override
  String get storeName => "breathing";

  @override
  BreathingEffect fromJson(Map<String, dynamic> json) =>
      _$BreathingEffectFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$BreathingEffectToJson(this);
}

@JsonSerializable()
class RainbowEffect extends LedEffect implements StorableObject {
  @override
  EffectType get type => EffectType.rainbow;

  double saturation;
  double value;
  double step;

  RainbowEffect(
      {required this.saturation, required this.value, required this.step});

  @override
  String get graphqlMutation => """
    mutation SetLedRainbow {
      rainbow(s: $saturation, v: $value, step: $step)
    }
  """;

  @override
  String get storeName => "rainbow";

  @override
  RainbowEffect fromJson(Map<String, dynamic> json) =>
      _$RainbowEffectFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$RainbowEffectToJson(this);
}
