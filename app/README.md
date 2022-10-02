## App

The app is used to call the [server](../server).

It gathers its IP using UDP discovery, currently via broadcast.

In the emulator, however, this doesn't work, since the emulator resides in a different subnet, so a
debug-only variable can be defined by running `flutter run`
with `--dart-define=RUSTY_DEBUG_SERVER_IP=your-ip`. (by default is 127.0.0.1)

## Build

Run `flutter pub run build_runner build` before the actual build, to run the code generators. (like
the JSON serializable objects)

### Note while debugging / developing

The hardcoded/config IP is provided only at startup, so if the server goes down, the app needs to be
restarted.

This was accepted to avoid adding complex debug-only code. 
