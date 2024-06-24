# orbit2-sys

## About

This is a Rust binding of the Gnome ORBit2 corba library. It requires ORBit2-devel AND clang-devel to be installed on your system.

## Installing Orbit2 & clang-devel

### On RPM based distributions

```sh
dnf install -y epel-release which && dnf clean all
dnf install -y ORBit2-devel clang-devel && dnf clean all
```

### Other

You can find the Orbit2 source code here: <https://github.com/Distrotech/ORBit2>

## WIP Stuff

This is following orbit.pdf tutorial:

<https://github.com/jeteve/orbit-sys/blob/main/orbit.pdf>
