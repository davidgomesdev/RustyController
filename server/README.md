## Running the server

1. install libusb: `sudo apt-get install libudev-dev libsystemd-dev libusb-1.0-0-dev`
2. give permission for the move controller(s) to your user
    1. create a group called psmove (`sudo groupadd psmove`)
    2. give permission to access the USB devices of PS:
       1. create the file
       `/etc/udev/rules.d/10-psmove-hidraw-permissions.rules` with:
        
        `KERNEL=="hidraw*", SUBSYSTEM=="hidraw", MODE="0660", GROUP="psmove"`
    3. add your user to that group `sudo usermod -a -G psmove your_username`

*Note: On Ubuntu, you might need to run `rfkill unblock bluetooth`.*

Then just `export RUST_LOG=info,rusty_controller=debug; cargo run`.

## Pairing

Due to lack of a bluetooth library in Rust, the pairing isn't implemented. (currently there are bluetooth low-energy
libraries, but those don't use psmove's version of bluetooth)

The pairing is done manually with [psmoveapi](https://github.com/thp/psmoveapi).

## Windows limitation

The GraphQL subscription and controller updates are very slow on Windows.

## Auto-update

Currently, there's a GitHub action that runs on every `main` branch push, building for Ubuntu, and replacing the latest
build in `releases`.

The [auto-update](scripts/auto-update.sh) script, clones/pulls `main`, builds the binary.

If the build newer, runs [run-server.sh](scripts/run-server.sh), which re-launches Rusty.

_Note: This whole process achieves minimal downtime, by launching only after having it built._

You can run it every midnight or so, by adding the following line on: `crontab -e`.

```bash
0 0 * * * ( cd /home/kali/RustyController && cp server/scripts/auto-update.sh /tmp/rusty-auto-update.sh && bash /tmp/rusty-auto-update.sh >> /var/log/rusty-controller/auto_update.log 2>&1 )
```

## Grafana stack

To launch Grafana (along with Loki and Prometheus), run `docker compose -f grafana.yaml up -d` on the [docker](docker) folder.

To get the host machine's metrics, you need to run [node_exporter](https://github.com/prometheus/node_exporter). (a simple binary that exposes the metrics via an endpoint)

### Logs

That cron logs to `/var/log/rusty-controller`, if you're running as a non-sudo user, you need to create and give
permission to that folder
beforehand.

`sudo mkdir -p /var/log/rusty-controller && sudo chown -R your_user:your_group /var/log/rusty-controller`
