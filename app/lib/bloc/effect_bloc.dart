import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';

import '../service/controller_service.dart';

class EffectBloc extends Bloc<EffectChangeEvent, LedEffect> {
  EffectBloc(LedEffect effect) : super(effect) {
    final service = serviceLocator.get<ControllerService>();

    on<EffectSettingChangeEvent>((event, emit) async {
      await service.set(effect: event.effect);
    });
    on<EffectTypeChangeEvent>((event, emit) async {
      final effect = await service.get(type: event.type);

      await service.set(effect: effect);

      emit(effect);
    });
  }
}

abstract class EffectChangeEvent {}

class EffectSettingChangeEvent extends EffectChangeEvent {
  LedEffect effect;

  EffectSettingChangeEvent(this.effect);
}

class EffectTypeChangeEvent extends EffectChangeEvent {
  EffectType type;

  EffectTypeChangeEvent(this.type);
}
