# Software

## Preparation

Install rust:

```
(hm - wie machen wir das in schön?)
```

## Compiling

### Debug

```sh
cd software/
cargo b
```

### release

When building in release mode the static files (frontend/static/) will be included in the final binary.

```sh
cd software/
cargo b --release
```

When compiling on ARM (for example Raspberry Pi), the project will automaticly be built for that target.

## Cross compiling

Targets for different RaspberryPi models are defined in .cargo/config.

For example compile for RaspberryPi 4:
```
cd software/
cargo b --release --target=armv7-unknown-linux-musleabihf
```

