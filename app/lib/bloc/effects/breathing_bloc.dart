import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:rusty_controller/bloc/specific_effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class BreathingBloc
    extends SpecificEffectBloc<BreathingEffectEvent, BreathingLedEffect> {
  BreathingBloc(super.effect) {
    on<BreathingEffectEvent>((event, emit) => emit(event.toEffect(state)));
  }
}

class BreathingEffectEvent {
  HSVColor? color;
  int? timeToPeak;
  double? peak;
  bool? breatheFromOff;

  BreathingEffectEvent(
      {this.color, this.timeToPeak, this.peak, this.breatheFromOff});

  BreathingLedEffect toEffect(BreathingLedEffect currentEffect) {
    HSVColor color = this.color ?? currentEffect.color;

    if (breatheFromOff == true) {
      color = color.withValue(1.0);
    }

    return BreathingLedEffect(
        color: color,
        timeToPeak: timeToPeak ?? currentEffect.timeToPeak,
        peak: peak ?? currentEffect.peak,
        breatheFromOff: breatheFromOff ?? currentEffect.breatheFromOff);
  }
}
