# lg600-mapepr

A simple key remapper for the Logitech LG600 mouse on Linux, written in Rust.

It works at the evdev level: the program takes exclusive control ("grabs") of the
mouse's input device, reads its events, and re-emits them through a virtual
uinput device. Remapping is meant to happen in between, so any button on the
mouse can be turned into a different key or button before the rest of the system
sees it.

## Status

Early work in progress. At the moment the program only grabs the mouse and
forwards its key/button events unchanged to a virtual device named
`LG600_VRT_KEYBOARD`. The actual remapping logic, and configuration for it,
are not implemented yet.

Known limitations:

- The input device path is hardcoded to `/dev/input/event10`. Your mouse may
  show up under a different `eventN` number.
- The virtual device is only created with the mouse's key/button capabilities,
  so pointer movement and scrolling are not forwarded yet.
- Errors when emitting events are ignored.

## Requirements

- Linux with evdev and uinput support (the `uinput` kernel module loaded)
- Rust toolchain (edition 2024, so a recent stable release)
- Permission to read `/dev/input/event*` and write `/dev/uinput`. This usually
  means running as root or being in the `input` group with a suitable udev rule.

## Building and running

```bash
cargo build --release
sudo ./target/release/lg600-mapepr
```

Or during development:

```bash
sudo cargo run
```

To find which event device belongs to your mouse, check
`/proc/bus/input/devices` or run `sudo evtest`, then update the path in
`src/main.rs`.

While the program runs, the physical mouse is grabbed, so other applications
only receive input through the virtual device. Stop it with Ctrl+C to release
the mouse.

## Dependencies

- [`evdev`](https://crates.io/crates/evdev) 0.13 for reading input events and
  creating the virtual uinput device.

## Layout

- `src/main.rs`: opens and grabs the device, creates the virtual device, and
  runs the event loop.
