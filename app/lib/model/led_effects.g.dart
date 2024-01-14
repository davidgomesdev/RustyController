// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'led_effects.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

StaticLedEffect _$StaticLedEffectFromJson(Map<String, dynamic> json) =>
    StaticLedEffect(
      color: const HSVColorJsonConverter()
          .fromJson(json['color'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$StaticLedEffectToJson(StaticLedEffect instance) =>
    <String, dynamic>{
      'color': const HSVColorJsonConverter().toJson(instance.color),
    };

BreathingLedEffect _$BreathingLedEffectFromJson(Map<String, dynamic> json) =>
    BreathingLedEffect(
      color: const HSVColorJsonConverter()
          .fromJson(json['color'] as Map<String, dynamic>),
      timeToPeak: json['timeToPeak'] as int,
      peak: (json['peak'] as num).toDouble(),
      breatheFromOff: json['breatheFromOff'] as bool,
    );

Map<String, dynamic> _$BreathingLedEffectToJson(BreathingLedEffect instance) =>
    <String, dynamic>{
      'color': const HSVColorJsonConverter().toJson(instance.color),
      'timeToPeak': instance.timeToPeak,
      'peak': instance.peak,
      'breatheFromOff': instance.breatheFromOff,
    };

CandleLedEffect _$CandleLedEffectFromJson(Map<String, dynamic> json) =>
    CandleLedEffect(
      hue: (json['hue'] as num).toDouble(),
      saturation: (json['saturation'] as num).toDouble(),
      minValue: (json['minValue'] as num).toDouble(),
      maxValue: (json['maxValue'] as num).toDouble(),
      variability: (json['variability'] as num).toDouble(),
      interval: json['interval'] as int,
    );

Map<String, dynamic> _$CandleLedEffectToJson(CandleLedEffect instance) =>
    <String, dynamic>{
      'hue': instance.hue,
      'saturation': instance.saturation,
      'minValue': instance.minValue,
      'maxValue': instance.maxValue,
      'variability': instance.variability,
      'interval': instance.interval,
    };

RainbowLedEffect _$RainbowLedEffectFromJson(Map<String, dynamic> json) =>
    RainbowLedEffect(
      saturation: (json['saturation'] as num).toDouble(),
      value: (json['value'] as num).toDouble(),
      timeToComplete: (json['timeToComplete'] as num).toDouble(),
    );

Map<String, dynamic> _$RainbowLedEffectToJson(RainbowLedEffect instance) =>
    <String, dynamic>{
      'saturation': instance.saturation,
      'value': instance.value,
      'timeToComplete': instance.timeToComplete,
    };
