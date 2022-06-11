import 'package:flutter/painting.dart';

abstract class LedEffect {
  EffectType get type;

  String get name => type.name;

  String get graphqlMutation;
}

enum EffectType { off, static, breathing, rainbow }

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

class StaticEffect extends LedEffect {
  @override
  EffectType get type => EffectType.static;

  HSVColor color;

  @override
  String get graphqlMutation => """
    mutation SetLedStatic {
      static(h: ${color.hue}, s: ${color.saturation}, v: ${color.value})
    }
  """;

  StaticEffect({required this.color});
}

class BreathingEffect extends LedEffect {
  @override
  EffectType get type => EffectType.breathing;

  HSVColor color;
  double step;
  double peak;

  @override
  String get graphqlMutation => """
    mutation SetLedBreathing {
      breathing(h: ${color.hue}, s: ${color.saturation}, initialV: ${color.value}, step: $step, peak: $peak)
    }
  """;

  BreathingEffect(
      {required this.color, required this.step, required this.peak});
}

class RainbowEffect extends LedEffect {
  @override
  EffectType get type => EffectType.rainbow;

  double saturation;
  double value;
  double step;

  @override
  String get graphqlMutation => """
    mutation SetLedRainbow {
      rainbow(s: $saturation, v: $value, step: $step)
    }
  """;

  RainbowEffect(
      {required this.saturation, required this.value, required this.step});
}
