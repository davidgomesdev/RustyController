import 'package:flutter/widgets.dart';
import 'package:rusty_controller/bloc/specific_effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class StaticBloc
    extends SpecificEffectBloc<StaticEffectEvent, StaticLedEffect> {
  StaticBloc(super.effect) {
    on<StaticEffectEvent>(
        (event, emit) => emit(StaticLedEffect(color: event.currentColor)));
  }
}

class StaticEffectEvent {
  HSVColor currentColor;

  double get initialValue => currentColor.value;

  StaticEffectEvent(this.currentColor);
}
