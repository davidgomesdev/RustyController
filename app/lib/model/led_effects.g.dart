// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'led_effects.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

StaticEffect _$StaticEffectFromJson(Map<String, dynamic> json) => StaticEffect(
      color: const HSVColorJsonConverter()
          .fromJson(json['color'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$StaticEffectToJson(StaticEffect instance) =>
    <String, dynamic>{
      'color': const HSVColorJsonConverter().toJson(instance.color),
    };

BreathingEffect _$BreathingEffectFromJson(Map<String, dynamic> json) =>
    BreathingEffect(
      color: const HSVColorJsonConverter()
          .fromJson(json['color'] as Map<String, dynamic>),
      step: (json['step'] as num).toDouble(),
      peak: (json['peak'] as num).toDouble(),
      breatheFromOff: json['breatheFromOff'] as bool,
    );

Map<String, dynamic> _$BreathingEffectToJson(BreathingEffect instance) =>
    <String, dynamic>{
      'color': const HSVColorJsonConverter().toJson(instance.color),
      'step': instance.step,
      'peak': instance.peak,
      'breatheFromOff': instance.breatheFromOff,
    };

RainbowEffect _$RainbowEffectFromJson(Map<String, dynamic> json) =>
    RainbowEffect(
      saturation: (json['saturation'] as num).toDouble(),
      value: (json['value'] as num).toDouble(),
      step: (json['step'] as num).toDouble(),
    );

Map<String, dynamic> _$RainbowEffectToJson(RainbowEffect instance) =>
    <String, dynamic>{
      'saturation': instance.saturation,
      'value': instance.value,
      'step': instance.step,
    };
