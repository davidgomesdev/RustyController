## Running the server

1. install libusb: `sudo apt-get install libusb-1.0-0-dev`
2. give permission for the move controller(s) to your user
   1. create a group called psmove (`sudo groupadd psmove`)
   2. give permission to access the USB devices of PS: create a file
      /etc/udev/rules.d/10-psmove-hidraw-permissions.rules`, with this line:
      2. `SUBSYSTEM=="usb", ATTR{idVendor}=="054c", MODE="0660", GROUP="psmove"`
   3. add your user to that group `sudo usermod -a -G psmove your_username`

Then just `export RUST_LOG=info,rusty_controller=debug; cargo run`.
