import 'dart:async';

import 'package:flutter/material.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';

import '../bloc/events/led_effects.dart';

final List<EffectEvent> effects = [
  OffEffectEvent(),
  StaticEffectEvent(color: Colors.black.toHSV()),
  BreathingEffectEvent(),
  RainbowEffectEvent(),
];

class EffectChooser extends StatelessWidget {
  final EffectEvent currentEffect;
  final StreamSink<EffectEvent> choiceStream;

  const EffectChooser(
      {Key? key, required this.choiceStream, required this.currentEffect})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        ...effects.map(
          (effect) => EffectChoice(
            choiceStream: choiceStream,
            effect: effect,
            isSelected: effect.type == currentEffect.type,
          ),
        ),
      ],
    );
  }
}

class EffectChoice extends StatelessWidget {
  final EffectEvent effect;
  final bool isSelected;
  final StreamSink<EffectEvent> choiceStream;

  const EffectChoice(
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
              onChanged: (_) {
                choiceStream.add(effect);
              },
            ),
            Text(effect.name),
          ],
        ),
      ),
    );
  }
}
