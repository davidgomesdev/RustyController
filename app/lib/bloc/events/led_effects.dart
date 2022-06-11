import 'package:flutter/painting.dart';

abstract class EffectEvent {
  EffectType get type;
  String get name => type.name;

  String get graphqlMutation;
}

enum EffectType { off, static, breathing, rainbow }

class OffEffectEvent extends EffectEvent {
  @override
  EffectType get type => EffectType.off;

  @override
  String get graphqlMutation => """
    mutation SetLedOff {
      off
    }
  """;
}

class StaticEffectEvent extends EffectEvent {
  @override
  EffectType get type => EffectType.static;

  HSVColor color;

  @override
  String get graphqlMutation => """
    mutation SetLedStatic {
      static(h: ${color.hue}, s: ${color.saturation}, v: ${color.value})
    }
  """;

  StaticEffectEvent({required this.color});
}

// TODO
class BreathingEffectEvent extends EffectEvent {
  @override
  EffectType get type => EffectType.breathing;

  @override
  String get graphqlMutation => """
    mutation SetLedBreathing {
      off
    }
  """;
}

// TODO
class RainbowEffectEvent extends EffectEvent {
  @override
  EffectType get type => EffectType.rainbow;

  @override
  String get graphqlMutation => """
    mutation SetLedRainbow {
      off
    }
  """;
}
