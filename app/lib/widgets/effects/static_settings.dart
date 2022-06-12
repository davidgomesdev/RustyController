import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/bloc/effects/static_bloc.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effects/common/led_color_picker.dart';

class StaticEffectWidget extends StatelessWidget {
  final StaticEffect currentEffect;

  const StaticEffectWidget({Key? key, required this.currentEffect})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    final bloc = serviceLocator.get<StaticBloc>();

    return BlocBuilder<StaticBloc, StaticEffect>(
      bloc: bloc,
      builder: (_, effect) => LedColorPicker(
        currentColor: effect.color,
        onColorPick: (color) => bloc.add(StaticColorEvent(color)),
      ),
    );
  }
}
