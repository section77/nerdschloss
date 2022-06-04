# nerdschloss

[Wiki](https://wiki.section77.de/de/projekte/nerdschloss/nerdschloss-reloaded)

## Preparation

Install rust:

```(hm - wie machen wir das in schön?)
```

## Compiling for Raspberry Pi

When on Raspberry Pi, simply run with default features:

```sh
cd software/backend/
cargo run
```

This includes the `hardware` feature to control the motor.

## Testing/developing on other machines

To execute on another machine without GPIO pins, compile without the `hardware` feature:

```sh
cd backend/
cargo run --no-default-features
```

A simulated motor will appear, that just shows output for open/close events on stdout and waits 5 seconds for each of these actions.
