import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/bloc/effect_bloc.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_chooser.dart';
import 'package:rusty_controller/widgets/effect_widget.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final bloc = serviceLocator.get<EffectBloc>();

    return BlocBuilder<EffectBloc, LedEffect>(
      bloc: bloc,
      builder: (_, effect) {
        return Row(
          children: [
            Expanded(
              child: EffectChooser(currentEffect: effect, bloc: bloc),
            ),
            Expanded(
              flex: 3,
              child: EffectWidget(effect, bloc: bloc),
            ),
          ],
        );
      },
    );
  }
}
