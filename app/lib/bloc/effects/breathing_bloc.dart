import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:rusty_controller/bloc/specific_effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class BreathingBloc
    extends SpecificEffectBloc<BreathingEffectEvent, BreathingLedEffect> {
  BreathingBloc(BreathingLedEffect effect) : super(effect) {
    on<BreathingColorEvent>(
        (event, emit) => emit(state..color = event.currentColor));
    on<BreathingStepEvent>((event, emit) => emit(state..step = event.step));
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

class BreathingStepEvent extends BreathingEffectEvent {
  double step;

  BreathingStepEvent(this.step);
}

class BreathingPeakEvent extends BreathingEffectEvent {
  double peak;

  BreathingPeakEvent(this.peak);
}

class BreathingFromOffEvent extends BreathingEffectEvent {
  bool breatheFromOff;

  BreathingFromOffEvent(this.breatheFromOff);
}
