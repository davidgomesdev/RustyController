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
      step: (json['step'] as num).toDouble(),
      peak: (json['peak'] as num).toDouble(),
      breatheFromOff: json['breatheFromOff'] as bool,
    );

Map<String, dynamic> _$BreathingLedEffectToJson(BreathingLedEffect instance) =>
    <String, dynamic>{
      'color': const HSVColorJsonConverter().toJson(instance.color),
      'step': instance.step,
      'peak': instance.peak,
      'breatheFromOff': instance.breatheFromOff,
    };

RainbowLedEffect _$RainbowLedEffectFromJson(Map<String, dynamic> json) =>
    RainbowLedEffect(
      saturation: (json['saturation'] as num).toDouble(),
      value: (json['value'] as num).toDouble(),
      step: (json['step'] as num).toDouble(),
    );

Map<String, dynamic> _$RainbowLedEffectToJson(RainbowLedEffect instance) =>
    <String, dynamic>{
      'saturation': instance.saturation,
      'value': instance.value,
      'step': instance.step,
    };
