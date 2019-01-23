# Puma OS

Following along the amazing blog by Philipp Oppermann.

https://os.phil-opp.com/

## Prerequisites

* Rustup
* QEMU

## Make sure the compiler is set to the 'nightly' version

```
> rustup override add nightly
```

## Building

bootimage must be installed first

```
> cargo install bootimage --version "^0.5.0"
```

```
> bootimage build
```

## Run via QEMU

```
> bootimage run
```