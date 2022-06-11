import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class StaticBloc extends Bloc<StaticEffectEvent, StaticEffect> {
  StaticBloc(StaticEffect effect) : super(effect) {
    on<StaticColorEvent>(
        (event, emit) => emit(state..color = event.currentColor));
  }
}

abstract class StaticEffectEvent {}

class StaticColorEvent extends StaticEffectEvent {
  HSVColor currentColor;

  double get initialValue => currentColor.value;

  StaticColorEvent(this.currentColor);
}
