import 'package:flutter/painting.dart';

abstract class EffectEvent {
  EffectEventType get type;
  String get name => type.name;

  String get graphqlMutation;
}

enum EffectEventType { off, static, breathing, rainbow }

class OffEffectEvent extends EffectEvent {
  @override
  EffectEventType get type => EffectEventType.off;

  @override
  String get graphqlMutation => """
    mutation SetLedOff {
      off
    }
  """;
}

class StaticEffectEvent extends EffectEvent {
  @override
  EffectEventType get type => EffectEventType.static;

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
  EffectEventType get type => EffectEventType.breathing;

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
  EffectEventType get type => EffectEventType.rainbow;

  @override
  String get graphqlMutation => """
    mutation SetLedRainbow {
      off
    }
  """;
}
