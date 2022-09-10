## App

The app is used to call the [server](../server).

It gathers its IP using UDP discovery, currently via broadcast.

In the emulator, however, this doesn't work, since the emulator resides in a different subnet, so a
debug-only variable can be defined by running `flutter run`
with `--dart-define=RUSTY_DEBUG_SERVER_IP=your-ip`. (by default is 127.0.0.1)
