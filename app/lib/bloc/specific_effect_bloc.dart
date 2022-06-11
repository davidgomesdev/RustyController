import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/bloc/effect_bloc.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';

abstract class SpecificEffectBloc<EffectEvent, State extends LedEffect>
    extends Bloc<EffectEvent, State> {
  SpecificEffectBloc(State initialState) : super(initialState);

  @override
  void on<E extends EffectEvent>(
    EventHandler<E, State> handler, {
    EventTransformer<E>? transformer,
  }) {
    super.on<E>((event, emit) {
      handler(event, emit);

      serviceLocator.get<EffectBloc>().add(EffectSettingChangeEvent(state));
    }, transformer: transformer);
  }
}
