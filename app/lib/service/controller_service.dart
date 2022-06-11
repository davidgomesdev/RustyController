import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:rusty_controller/main.dart';
import 'package:rusty_controller/model/led_effects.dart';

class ControllerService {
// TODO: use UDP discovery
  final _graphqlClient = GraphQLClient(
    link: HttpLink("http://127.0.0.1:8080/graphql"),
    cache: GraphQLCache(store: InMemoryStore()),
  );

  void set({required LedEffect effect}) {
    log.i("Setting effect to '${effect.name}'");

    _graphqlClient
        .mutate(
          MutationOptions(document: gql(effect.graphqlMutation)),
        )
        .then(log.v, onError: (msg, _) => log.e(msg));
  }
}
