import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class BreathingBloc extends Bloc<BreathingEffectEvent, BreathingEffect> {
  BreathingBloc(BreathingEffect effect) : super(effect) {
    on<BreathingColorEvent>((event, emit) {
      if (event.initialValue > state.peak) {
        state.peak = event.initialValue;
      }

      emit(state..color = event.currentColor);
    });
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
