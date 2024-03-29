import 'package:flutter/material.dart';
import 'package:get/get_navigation/src/root/get_material_app.dart';
import 'package:get_it/get_it.dart';
import 'package:logger/logger.dart';
import 'package:rusty_controller/bloc/discovery_bloc.dart';
import 'package:rusty_controller/bloc/effect_bloc.dart';
import 'package:rusty_controller/bloc/effects/breathing_bloc.dart';
import 'package:rusty_controller/bloc/effects/candle_bloc.dart';
import 'package:rusty_controller/bloc/effects/rainbow_bloc.dart';
import 'package:rusty_controller/bloc/effects/static_bloc.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';
import 'package:rusty_controller/global_consts.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/screen/home_screen.dart';
import 'package:rusty_controller/service/controller_service.dart';
import 'package:rusty_controller/service/discovery_service.dart';
import 'package:rusty_controller/service/store_service.dart';

final log = Logger(level: Level.trace, printer: PrettyPrinter());
final serviceLocator = GetIt.instance;

final defaultEffects = {
  EffectType.off: OffLedEffect(),
  EffectType.static: StaticLedEffect(color: Colors.black.toHSV()),
  EffectType.breathing: BreathingLedEffect(
    color: Colors.red.toHSV().withValue(0.0),
    timeToPeak: maxBreathingTime,
    peak: 1.0,
    breatheFromOff: true,
  ),
  EffectType.candle: CandleLedEffect(
    hue: 0,
    saturation: 1.0,
    minValue: 0.5,
    maxValue: 0.8,
    variability: 1.0,
    interval: 100,
  ),
  EffectType.rainbow: RainbowLedEffect(
    saturation: 1.0,
    value: 0.5,
    timeToComplete: maxRainbowTime,
  ),
};

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
          defaultValue: defaultEffects[EffectType.static] as StaticLedEffect);

      return StaticBloc(savedStatic);
    },
  );
  serviceLocator.registerSingletonAsync(
    () async {
      final savedBreathing = await storeService.get<BreathingLedEffect>(
          defaultValue:
              defaultEffects[EffectType.breathing] as BreathingLedEffect);

      return BreathingBloc(savedBreathing);
    },
  );
  serviceLocator.registerSingletonAsync(
    () async {
      final savedCandle = await storeService.get<CandleLedEffect>(
          defaultValue: defaultEffects[EffectType.candle] as CandleLedEffect);

      return CandleBloc(savedCandle);
    },
  );
  serviceLocator.registerSingletonAsync(
    () async {
      final savedRainbow = await storeService.get<RainbowLedEffect>(
          defaultValue: defaultEffects[EffectType.rainbow] as RainbowLedEffect);

      return RainbowBloc(savedRainbow);
    },
  );
}

class BaseScreen extends StatelessWidget {
  const BaseScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return GetMaterialApp(
      themeMode: ThemeMode.dark,
      darkTheme: ThemeData.dark(useMaterial3: true),
      home: const Scaffold(
        body: ScaffoldMessenger(
          child: SafeArea(child: HomeScreen()),
        ),
      ),
    );
  }
}
