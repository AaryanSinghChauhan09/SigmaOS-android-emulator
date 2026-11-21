# SigmaOS-android-emulator

SigmaOS integration component for **Android emulator**.

## Commands

```bash
sigmaos-android-emulator install   # install package + deploy defaults
sigmaos-android-emulator deploy    # copy config templates to $HOME
sigmaos-android-emulator status    # binary/package/config report
sigmaos-android-emulator reset     # remove deployed config
sigmaos-android-emulator open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
