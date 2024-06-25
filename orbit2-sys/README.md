# orbit2-sys

## About

This is a Rust binding of the Gnome ORBit2 corba library. It requires ORBit2-devel AND clang-devel to be installed on your system.

This is highly experimental and subject to uncontrolled change. Use at your own perils or get in touch with the author (Jerome Eteve)

The source code is there: <https://github.com/jeteve/orbit2-rs>

## Installing Orbit2 & clang-devel

### On RPM based distributions

See <https://pkgs.org/search/?q=orbit2>

```sh
dnf install -y epel-release which && dnf clean all
dnf install -y ORBit2-devel clang-devel && dnf clean all
```

### Other

You can find the Orbit2 source code here: <https://github.com/Distrotech/ORBit2>

Additionally, debian based systems used to have the lib:

See
<https://sources.debian.org/src/orbit2/>

and

<https://tracker.debian.org/pkg/orbit2>

## Checking prerequisites

You should be able to do `pkg-config  ORBit-2.0 --cflags --libs --static`

## WIP Stuff

This is following orbit.pdf tutorial:

<https://github.com/jeteve/orbit-sys/blob/main/orbit.pdf>
