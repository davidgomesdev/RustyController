import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rate_limiter/rate_limiter.dart';
import 'package:rusty_controller/bloc/effect_bloc.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';

abstract class SpecificEffectBloc<EffectEvent, State extends LedEffect>
    extends Bloc<EffectEvent, State> {
  final addEvent = throttle(
    serviceLocator.get<EffectBloc>().add,
    const Duration(milliseconds: 100),
  );

  SpecificEffectBloc(State initialState) : super(initialState);

  @override
  void on<E extends EffectEvent>(
    EventHandler<E, State> handler, {
    EventTransformer<E>? transformer,
  }) {
    super.on<E>((event, emit) {
      handler(event, emit);

      addEvent([EffectSettingChangeEvent(state)]);
    }, transformer: transformer);
  }
}
