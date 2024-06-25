# orbit2-rs

## About

A Rust toolkit to enable using the Corba ORBit2 library. ( <https://github.com/Distrotech/ORBit2> ).

ORBit2 is legacy software. You can still find the documentation online thanks to archive.org:

<https://web.archive.org/web/20120525221306/http://projects.gnome.org/ORBit2/documentation.html>

This is very experimental and subjected to uncontrolled changes. So use it at your own perils.

## Project structure

## Modules

### orbit2-sys

The orbit2_sys::core::* types and binding to the orbit2 C library

### orbit2-buildtools

The orbit2 build tools, to enable building:

- An orbit2 based service common/client side based on the orbit2-sys library, build from an idl. This should be fully automated as there is NO code to implenment

- An orbit2 based service implementation. Based on the common. This should provide some tooling to make it easier to implement the service inside your server application.

### orbit2

The more idiomatic interface to orbit2-sys and some helper functions to help with gluing
the client libs generated with orbit2-buildtools.

### orbit2-sample-idls

Sample IDLs module, build from sample IDLs from the orbit2 documentation.
