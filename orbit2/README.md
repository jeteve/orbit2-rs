# orbit2

An idiomatic Rust layer on top of orbit2-sys

## Implementing Corba clients lib and services

See crate `orbit2-buildtools`.

## Notes from the original ORBit doc

Note: it is possible to use a third-party naming service, e.g. using the --ORBNamingIOR command
line option.

Note: From ORBit 0.5.3, IIOP communications over IP sockets are disabled by default for security reasons.
You should create an /etc/orbitrc (for a system-wide settings), or an ~/.orbitrc (for one user’s
setting) containing something like :
ORBIIOPUSock=1
ORBIIOPIPv4=1
ORBIIOPIPv6=0

Note: resolves default name-service, usually given to application as

* command line argument "-ORBInitRef NameService=IOR:0100000028..",
* or since release 2.8.0 corbalocs in form of URL can be used, eg:
* "-ORBInitRef NameService=corbaloc:iiop:HOSTNAME:PORT/NameService%00"

## TODOs

Deal with exceptions correctly. See orbit-docs/orbit-docs/orbit/x894.html#AEN897
