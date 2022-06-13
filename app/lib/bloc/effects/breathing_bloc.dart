import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:rusty_controller/bloc/specific_effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class BreathingBloc
    extends SpecificEffectBloc<BreathingEffectEvent, BreathingEffect> {
  BreathingBloc(BreathingEffect effect) : super(effect) {
    on<BreathingColorEvent>(
        (event, emit) => emit(state..color = event.currentColor));
    on<BreathingStepEvent>((event, emit) => emit(state..step = event.step));
    on<BreathingPeakEvent>((event, emit) => emit(state..peak = event.peak));
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
