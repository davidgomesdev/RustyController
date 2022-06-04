import 'package:flutter/painting.dart';

abstract class EffectEvent {
  String get name;
  String get graphqlMutation;
}

class OffEffectEvent extends EffectEvent {
  @override
  String get name => "Off";

  @override
  String get graphqlMutation => """
    mutation SetLedOff {
      off
    }
  """;
}

class StaticEffectEvent extends EffectEvent {
  @override
  String get name => "Static";

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
  String get name => "Breathing";

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
  String get name => "Rainbow";

  @override
  String get graphqlMutation => """
    mutation SetLedRainbow {
      off
    }
  """;
}
