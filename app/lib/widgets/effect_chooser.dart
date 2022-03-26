import 'dart:async';

import 'package:flutter/material.dart';
import '../bloc/events/led_effects.dart';

final List<LedEffectEvent> effects = [
  OffLedEffectEvent(),
  StaticLedEffectEvent(color: Colors.black),
  BreathingLedEffectEvent(),
  RainbowLedEffectEvent(),
];

class EffectChooser extends StatelessWidget {
  final LedEffectEvent currentEffect;
  final StreamSink<LedEffectEvent> choiceStream;

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
            isSelected: effect.name == currentEffect.name,
          ),
        ),
      ],
    );
  }
}

class EffectChoice extends StatelessWidget {
  final LedEffectEvent effect;
  final bool isSelected;
  final StreamSink<LedEffectEvent> choiceStream;

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
        child: Padding(
          padding: EdgeInsets.zero,
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
      ),
    );
  }
}
