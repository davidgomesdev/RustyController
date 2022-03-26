import 'dart:ui';

abstract class LedEffectEvent {
  String get name;
}

class OffLedEffectEvent extends LedEffectEvent {
  @override
  String get name => "Off";
}

class StaticLedEffectEvent extends LedEffectEvent {
  @override
  String get name => "Static";

  Color color;

  StaticLedEffectEvent({required this.color});
}

class BreathingLedEffectEvent extends LedEffectEvent {
  @override
  String get name => "Breathing";
}

class RainbowLedEffectEvent extends LedEffectEvent {
  @override
  String get name => "Rainbow";
}
