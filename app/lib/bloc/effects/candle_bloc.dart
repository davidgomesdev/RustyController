import 'package:rusty_controller/bloc/specific_effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class CandleBloc
    extends SpecificEffectBloc<CandleEffectEvent, CandleLedEffect> {
  CandleBloc(super.effect) {
    on<CandleEffectEvent>((event, emit) => emit(event.toEffect(state)));
  }
}

class CandleEffectEvent {
  double? hue;
  double? saturation;
  double? minValue;
  double? maxValue;
  double? variability;
  int? interval;

  CandleEffectEvent(
      {this.hue,
      this.saturation,
      this.minValue,
      this.maxValue,
      this.variability,
      this.interval});

  CandleLedEffect toEffect(CandleLedEffect currentEffect) {
    return CandleLedEffect(
        hue: hue ?? currentEffect.hue,
        saturation: saturation ?? currentEffect.saturation,
        minValue: minValue ?? currentEffect.minValue,
        maxValue: maxValue ?? currentEffect.maxValue,
        variability: variability ?? currentEffect.variability,
        interval: interval ?? currentEffect.interval);
  }
}
