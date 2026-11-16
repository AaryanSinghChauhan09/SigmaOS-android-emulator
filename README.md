# Android Emulator

Android Emulator is a SigmaOS component providing a functional domain model for `android-emulator2` with validated state, deterministic storage, and typed errors.

## Description

Android Emulator emulator compatibility layer for SigmaOS

## Domain model

- `Emulator` — domain entity

## API

- `new()`

## Testing

```sh
cargo test
```

Tests are deterministic, isolated, and self-validating.
