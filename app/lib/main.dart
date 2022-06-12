import 'package:flutter/material.dart';
import 'package:get_it/get_it.dart';
import 'package:logger/logger.dart';
import 'package:rusty_controller/bloc/effect_bloc.dart';
import 'package:rusty_controller/bloc/effects/breathing_bloc.dart';
import 'package:rusty_controller/bloc/effects/static_bloc.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/screen/home_screen.dart';
import 'package:rusty_controller/service/controller_service.dart';

var log = Logger(level: Level.info, printer: PrettyPrinter());
var serviceLocator = GetIt.instance;

void main() {
  setupDependencies();

  runApp(const BaseScreen());
}

// TODO: this could be in its own file
void setupDependencies() {
  // Services
  serviceLocator.registerSingleton(ControllerService());

  // Effect Blocs
  serviceLocator.registerLazySingleton(
    () => EffectBloc(OffEffect()),
  );
  serviceLocator.registerLazySingleton(
    () => StaticBloc(StaticEffect(color: Colors.black.toHSV())),
  );
  serviceLocator.registerLazySingleton(
    () => BreathingBloc(
        BreathingEffect(color: Colors.black.toHSV(), step: 0.01, peak: 1.0)),
  );
}

class BaseScreen extends StatelessWidget {
  const BaseScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      home: Scaffold(
        body: ScaffoldMessenger(
          child: SafeArea(child: HomeScreen()),
        ),
      ),
    );
  }
}
