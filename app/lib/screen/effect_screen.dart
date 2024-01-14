import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/bloc/effect_bloc.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_chooser.dart';
import 'package:rusty_controller/widgets/effect_widget.dart';

class EffectScreen extends StatelessWidget {
  const EffectScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final bloc = serviceLocator.get<EffectBloc>();

    return BlocBuilder<EffectBloc, LedEffect>(
      bloc: bloc,
      builder: (_, effect) {
        return Row(
          children: [
            Expanded(
              child: Padding(
                padding: const EdgeInsets.only(left: 8.0),
                child: EffectChooser(currentEffect: effect, bloc: bloc),
              ),
            ),
            Flexible(
              flex: 3,
              child: Center(child: EffectWidget(effect)),
            ),
          ],
        );
      },
    );
  }
}
