import 'package:flutter/material.dart';
import 'package:rusty_controller/bloc/effect_bloc.dart';

import '../model/led_effects.dart';

class EffectChooser extends StatelessWidget {
  final LedEffect currentEffect;
  final EffectBloc bloc;

  const EffectChooser(
      {super.key, required this.bloc, required this.currentEffect});

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        ...EffectType.values.where((element) => element != EffectType.none).map(
              (type) => Flexible(
                child: _EffectChoice(
                  name: type.name,
                  isSelected: type == currentEffect.type,
                  onSelected: () => bloc.add(EffectTypeChangeEvent(type)),
                ),
              ),
            ),
      ],
    );
  }
}

class _EffectChoice extends StatelessWidget {
  final String name;
  final bool isSelected;
  final VoidCallback onSelected;

  const _EffectChoice(
      {required this.name, required this.isSelected, required this.onSelected});

  @override
  Widget build(BuildContext context) {
    return AbsorbPointer(
      absorbing: isSelected,
      child: InkWell(
        onTap: () {
          if (!isSelected) {
            onSelected();
          }
        },
        child: Row(
          children: <Widget>[
            Expanded(
              child: Radio<String>(
                groupValue: isSelected ? name : '',
                value: name,
                onChanged: (_) => onSelected(),
              ),
            ),
            Expanded(flex: 2, child: Text(name)),
          ],
        ),
      ),
    );
  }
}
