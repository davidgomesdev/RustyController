import 'package:rusty_controller/bloc/specific_effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class RainbowBloc
    extends SpecificEffectBloc<RainbowEffectEvent, RainbowLedEffect> {
  RainbowBloc(super.effect) {
    on<RainbowEffectEvent>((event, emit) => emit(event.toEffect(state)));
  }
}

class RainbowEffectEvent {
  double? saturation;
  double? value;
  double? timeToComplete;

  RainbowEffectEvent({this.saturation, this.value, this.timeToComplete});

  RainbowLedEffect toEffect(RainbowLedEffect currentEffect) {
    return RainbowLedEffect(
        saturation: saturation ?? currentEffect.saturation,
        value: value ?? currentEffect.value,
        timeToComplete: timeToComplete ?? currentEffect.timeToComplete);
  }
}
