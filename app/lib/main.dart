import 'dart:async';

import 'package:flutter/material.dart';
import 'package:rusty_controller/bloc/events/led_effects.dart';
import 'package:rusty_controller/widgets/effect_chooser.dart';
import 'package:rusty_controller/widgets/effects/static_effect_settings.dart';

void main() {
  runApp(MaterialApp(
    builder: (ctx, _) => HomeScreen(),
  ));
}

class HomeScreen extends StatelessWidget {
  final _effectChoiceController = StreamController<LedEffectEvent>();

  HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: ScaffoldMessenger(
        child: StreamBuilder<LedEffectEvent>(
          initialData: OffLedEffectEvent(),
          stream: _effectChoiceController.stream,
          builder: (ctx, snapshot) {
            if (!snapshot.hasData) {
              return const CircularProgressIndicator.adaptive();
            }

            if (snapshot.hasError) {
              final snackBar = SnackBar(
                content: const Text('Yay! A SnackBar!'),
                action: SnackBarAction(
                  label: 'Undo',
                  onPressed: () {
                    // Some code to undo the change.
                  },
                ),
              );

              ScaffoldMessenger.of(context).showSnackBar(snackBar);
            }

            final currentEffect = snapshot.data!;
            final settings =
                _getEffectSettings(currentEffect, _effectChoiceController.sink);

            return Row(
              children: [
                Expanded(
                  child: EffectChooser(
                      choiceStream: _effectChoiceController.sink,
                      currentEffect: currentEffect),
                ),
                Expanded(
                  child: settings,
                ),
              ],
            );
          },
        ),
      ),
    );
  }

  Widget _getEffectSettings(
      LedEffectEvent currentEffect, StreamSink<LedEffectEvent> colorStream) {
    if (currentEffect is StaticLedEffectEvent) {
      return StaticEffectSettings(
        effectStream: colorStream,
        currentEffect: currentEffect,
      );
    } else {
      return Container();
    }
  }
}
