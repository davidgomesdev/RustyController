## App

The app is used to call the [server](../server). It gather its IP using UDP discovery, currently
using broadcast.

In the emulator, this doesn't work, since the emulator resides in a different subnet. For this, set
your local IP in `lib/service/discovery_service.dart` constructor.
