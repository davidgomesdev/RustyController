import 'package:equatable/equatable.dart';
import 'package:flutter/painting.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:rusty_controller/model/util/json_converters.dart';
import 'package:rusty_controller/service/store_service.dart';

part 'led_effects.g.dart';

abstract class LedEffect extends Equatable {
  EffectType get type;

  String get name => type.name;

  String get graphqlMutation;

  String get graphqlMutationName;

  Map<String, dynamic> get graphqlVariables;
}

enum EffectType { none, off, static, breathing, candle, rainbow }

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

  @override
  List<Object?> get props => [];
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

  @override
  List<Object?> get props => [];
}

@JsonSerializable()
@HSVColorJsonConverter()
class StaticLedEffect extends LedEffect implements StorableObject {
  @override
  EffectType get type => EffectType.static;

  final HSVColor color;

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
  Map<String, dynamic> get graphqlVariables => {
        "hue": color.hue.toInt(),
        "saturation": color.saturation,
        "value": color.value
      };

  @override
  String get storeName => "static";

  @override
  StaticLedEffect fromJson(Map<String, dynamic> json) =>
      _$StaticLedEffectFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$StaticLedEffectToJson(this);

  @override
  List<Object?> get props => [color];
}

@JsonSerializable()
@HSVColorJsonConverter()
class BreathingLedEffect extends LedEffect implements StorableObject {
  @override
  EffectType get type => EffectType.breathing;

  final HSVColor color;
  final int timeToPeak;
  final double peak;
  final bool breatheFromOff;

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
        "hue": color.hue.toInt(),
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

  @override
  List<Object?> get props => [color, timeToPeak, peak, breatheFromOff];
}

@JsonSerializable()
@HSVColorJsonConverter()
class CandleLedEffect extends LedEffect implements StorableObject {
  @override
  EffectType get type => EffectType.candle;

  final double hue;
  final double saturation;
  final double minValue;
  final double maxValue;
  final double variability;
  final int interval;

  CandleLedEffect(
      {required this.hue,
      required this.saturation,
      required this.minValue,
      required this.maxValue,
      required this.variability,
      required this.interval});

  @override
  String get graphqlMutation => """
    mutation SetLedCandle(\$input: CandleLedEffectInput!) {
      setLedCandle(input: \$input)
    }
  """;

  @override
  String get graphqlMutationName => "setLedCandle";

  @override
  Map<String, dynamic> get graphqlVariables => {
        "hue": hue.round(),
        "saturation": saturation,
        "minValue": minValue,
        "maxValue": maxValue,
        "variability": variability,
        "interval": interval
      };

  @override
  String get storeName => "candle";

  @override
  CandleLedEffect fromJson(Map<String, dynamic> json) =>
      _$CandleLedEffectFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$CandleLedEffectToJson(this);

  @override
  List<Object?> get props =>
      [hue, saturation, minValue, maxValue, variability, interval];
}

@JsonSerializable()
class RainbowLedEffect extends LedEffect implements StorableObject {
  @override
  EffectType get type => EffectType.rainbow;

  final double saturation;
  final double value;
  final double timeToComplete;

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

  @override
  List<Object?> get props => [saturation, value, timeToComplete];
}
