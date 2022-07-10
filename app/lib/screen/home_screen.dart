import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:rusty_controller/bloc/discovery_bloc.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/screen/effect_screen.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final bloc = serviceLocator.get<DiscoveryBloc>();

    return BlocBuilder<DiscoveryBloc, RustyConnectionState>(
      bloc: bloc,
      builder: (_, state) {
        if (state is DisconnectedState) {
          return Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: const [
              Center(
                child: Text(
                  "Not connected to the server, re-trying... ",
                ),
              ),
              Center(
                child: Text(
                  "Make sure you are on the same network as rusty",
                ),
              ),
            ],
          );
        } else if (state is ConnectedState) {
          return const EffectScreen();
        }

        return Container();
      },
    );
  }
}
