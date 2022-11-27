import 'package:rusty_controller/bloc/specific_effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class RainbowBloc
    extends SpecificEffectBloc<RainbowEffectEvent, RainbowLedEffect> {
  RainbowBloc(RainbowLedEffect effect) : super(effect) {
    on<RainbowSaturationEvent>(
        (event, emit) => emit(state..saturation = event.saturation));
    on<RainbowValueEvent>((event, emit) => emit(state..value = event.value));
    on<RainbowTimeEvent>(
        (event, emit) => emit(state..timeToComplete = event.timeToComplete));
  }
}

abstract class RainbowEffectEvent {}

class RainbowSaturationEvent extends RainbowEffectEvent {
  double saturation;

  RainbowSaturationEvent(this.saturation);
}

class RainbowValueEvent extends RainbowEffectEvent {
  double value;

  RainbowValueEvent(this.value);
}

class RainbowTimeEvent extends RainbowEffectEvent {
  double timeToComplete;

  RainbowTimeEvent(this.timeToComplete);
}
