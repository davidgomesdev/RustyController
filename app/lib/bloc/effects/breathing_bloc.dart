import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:rusty_controller/bloc/specific_effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class BreathingBloc
    extends SpecificEffectBloc<BreathingEffectEvent, BreathingLedEffect> {
  BreathingBloc(BreathingLedEffect effect) : super(effect) {
    on<BreathingColorEvent>(
        (event, emit) => emit(state..color = event.currentColor));
    on<BreathingTimeEvent>(
        (event, emit) => emit(state..timeToPeak = event.timeToPeak));
    on<BreathingPeakEvent>((event, emit) => emit(state..peak = event.peak));
    on<BreathingFromOffEvent>((event, emit) {
      if (event.breatheFromOff) {
        emit(state..breatheFromOff = true);
      } else {
        emit(state
          ..breatheFromOff = false
          ..color = state.color.withValue(1.0));
      }
    });
  }
}

abstract class BreathingEffectEvent {}

class BreathingColorEvent extends BreathingEffectEvent {
  HSVColor currentColor;

  double get initialValue => currentColor.value;

  BreathingColorEvent(this.currentColor);
}

class BreathingTimeEvent extends BreathingEffectEvent {
  int timeToPeak;

  BreathingTimeEvent(this.timeToPeak);
}

class BreathingPeakEvent extends BreathingEffectEvent {
  double peak;

  BreathingPeakEvent(this.peak);
}

class BreathingFromOffEvent extends BreathingEffectEvent {
  bool breatheFromOff;

  BreathingFromOffEvent(this.breatheFromOff);
}
