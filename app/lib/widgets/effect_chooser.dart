import 'dart:async';

import 'package:flutter/material.dart';

import '../model/led_effects.dart';

class EffectChooser extends StatelessWidget {
  final LedEffect currentEffect;
  final StreamSink<LedEffect> choiceStream;
  final Map<EffectType, LedEffect> effects;

  const EffectChooser(
      {Key? key,
      required this.choiceStream,
      required this.currentEffect,
      required this.effects})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        ...effects.values.map(
          (effect) => _EffectChoice(
            choiceStream: choiceStream,
            effect: effect,
            isSelected: effect.type == currentEffect.type,
          ),
        ),
      ],
    );
  }
}

class _EffectChoice extends StatelessWidget {
  final LedEffect effect;
  final bool isSelected;
  final StreamSink<LedEffect> choiceStream;

  const _EffectChoice(
      {Key? key,
      required this.choiceStream,
      required this.effect,
      required this.isSelected})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return AbsorbPointer(
      absorbing: isSelected,
      child: InkWell(
        onTap: () {
          if (!isSelected) {
            choiceStream.add(effect);
          }
        },
        child: Row(
          children: <Widget>[
            Radio<String>(
              groupValue: isSelected ? effect.name : '',
              value: effect.name,
              onChanged: (_) => choiceStream.add(effect),
            ),
            Text(effect.name),
          ],
        ),
      ),
    );
  }
}
