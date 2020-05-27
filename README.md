# Puma OS

Following along the amazing blog by Philipp Oppermann.

https://os.phil-opp.com/

## Prerequisites

* Rustup
* QEMU
* rust-src
* llvm-tools-preview
* cargo-xbuild
* bootimage

## Make sure the compiler is set to the 'nightly' version

```
> rustup override add nightly
```

## Building

Install the prerequisites

```
> rustup component add rust-src
> rustup component add llvm-tools-preview
> cargo install cargo-xbuild
> cargo install bootimage --version "^0.8.0"
```

```
> cargo xbuild
```

## Run via QEMU

```
> cargo xrun
```
