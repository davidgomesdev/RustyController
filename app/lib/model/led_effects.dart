import 'package:flutter/painting.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:rusty_controller/model/util/json_converters.dart';
import 'package:rusty_controller/service/store_service.dart';

part 'led_effects.g.dart';

abstract class LedEffect {
  EffectType get type;

  String get name => type.name;

  String get graphqlMutation;

  String get graphqlMutationName;

  Map<String, dynamic> get graphqlVariables;

  StorableObject? get storeObject => null;
}

enum EffectType { none, off, static, breathing, rainbow }

// A placeholder/null-object pattern for when there's no effect selected,
// while avoiding null-checks
class NoLedEffect extends LedEffect {
  @override
  EffectType get type => EffectType.none;

  @override
  String get graphqlMutation => "";

  @override
  String get graphqlMutationName => "";

  @override
  Map<String, dynamic> get graphqlVariables => {};
}

class OffLedEffect extends LedEffect {
  @override
  EffectType get type => EffectType.off;

  @override
  String get graphqlMutation => """
    mutation SetLedOff {
      setLedOff
    }
  """;

  @override
  String get graphqlMutationName => "setLedOff";

  @override
  Map<String, dynamic> get graphqlVariables => {};
}

@JsonSerializable()
@HSVColorJsonConverter()
class StaticLedEffect extends LedEffect implements StorableObject {
  @override
  EffectType get type => EffectType.static;

  HSVColor color;

  StaticLedEffect({required this.color});

  @override
  String get graphqlMutation => """
    mutation SetLedStatic(\$input: StaticLedEffectInput!) {
      setLedStatic(input: \$input)
    }
  """;

  @override
  String get graphqlMutationName => "setLedStatic";

  @override
  Map<String, dynamic> get graphqlVariables =>
      {"hue": color.hue, "saturation": color.saturation, "value": color.value};

  @override
  String get storeName => "static";

  @override
  StaticLedEffect fromJson(Map<String, dynamic> json) =>
      _$StaticLedEffectFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$StaticLedEffectToJson(this);
}

@JsonSerializable()
@HSVColorJsonConverter()
class BreathingLedEffect extends LedEffect implements StorableObject {
  @override
  EffectType get type => EffectType.breathing;

  HSVColor color;
  int timeToPeak;
  double peak;
  bool breatheFromOff;

  BreathingLedEffect(
      {required this.color,
      required this.timeToPeak,
      required this.peak,
      required this.breatheFromOff});

  @override
  String get graphqlMutation => """
    mutation SetLedBreathing(\$input: BreathingLedEffectInput!) {
      setLedBreathing(input: \$input)
    }
  """;

  @override
  String get graphqlMutationName => "setLedBreathing";

  @override
  Map<String, dynamic> get graphqlVariables => {
        "hue": color.hue,
        "saturation": color.saturation,
        "initialValue": color.value,
        "timeToPeak": timeToPeak,
        "peak": peak
      };

  @override
  String get storeName => "breathing";

  @override
  BreathingLedEffect fromJson(Map<String, dynamic> json) =>
      _$BreathingLedEffectFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$BreathingLedEffectToJson(this);
}

@JsonSerializable()
class RainbowLedEffect extends LedEffect implements StorableObject {
  @override
  EffectType get type => EffectType.rainbow;

  double saturation;
  double value;
  double timeToComplete;

  RainbowLedEffect(
      {required this.saturation,
      required this.value,
      required this.timeToComplete});

  @override
  String get graphqlMutation => """
    mutation SetLedRainbow(\$input: RainbowLedEffectInput!) {
      setLedRainbow(input: \$input)
    }
  """;

  @override
  String get graphqlMutationName => "setLedRainbow";

  @override
  Map<String, dynamic> get graphqlVariables => {
        "saturation": saturation,
        "value": value,
        "timeToComplete": timeToComplete
      };

  @override
  String get storeName => "rainbow";

  @override
  RainbowLedEffect fromJson(Map<String, dynamic> json) =>
      _$RainbowLedEffectFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$RainbowLedEffectToJson(this);
}
