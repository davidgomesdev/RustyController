import 'package:flutter/material.dart';
import 'package:get_it/get_it.dart';
import 'package:logger/logger.dart';
import 'package:rusty_controller/bloc/discovery_bloc.dart';
import 'package:rusty_controller/bloc/effect_bloc.dart';
import 'package:rusty_controller/bloc/effects/breathing_bloc.dart';
import 'package:rusty_controller/bloc/effects/rainbow_bloc.dart';
import 'package:rusty_controller/bloc/effects/static_bloc.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';
import 'package:rusty_controller/global_consts.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/screen/home_screen.dart';
import 'package:rusty_controller/service/controller_service.dart';
import 'package:rusty_controller/service/discovery_service.dart';
import 'package:rusty_controller/service/store_service.dart';

var log = Logger(level: Level.verbose, printer: PrettyPrinter());
var serviceLocator = GetIt.instance;

void main() {
  setupDependencies();

  runApp(const BaseScreen());
}

void setupDependencies() {
  // Connection Bloc
  serviceLocator.registerSingleton(DiscoveryBloc());

  // Store service - to get saved effects
  final storeService = StoreService();
  serviceLocator.registerSingleton(storeService);

  // Remaining services
  serviceLocator.registerSingleton(DiscoveryService());
  serviceLocator.registerSingleton(ControllerService());

  // Effect Blocs
  serviceLocator.registerSingleton(
    EffectBloc(NoLedEffect()),
  );
  serviceLocator.registerSingletonAsync(
    () async {
      final savedStatic = await storeService.get(
          defaultValue: StaticLedEffect(color: Colors.black.toHSV()));

      return StaticBloc(savedStatic);
    },
  );
  serviceLocator.registerSingletonAsync(
    () async {
      final savedBreathing = await storeService.get(
          defaultValue: BreathingLedEffect(
              color: Colors.black.toHSV(),
              step: maxBreathingStep,
              peak: 1.0,
              breatheFromOff: true));

      return BreathingBloc(savedBreathing);
    },
  );
  serviceLocator.registerSingletonAsync(
    () async {
      final savedRainbow = await storeService.get(
          defaultValue: RainbowLedEffect(
              saturation: 1.0, value: 1.0, step: maxRainbowStep));

      return RainbowBloc(savedRainbow);
    },
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
