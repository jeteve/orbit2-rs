# Notes.

## Modules

### orbit2-sys

The orbit2::core::* types and binding to the orbit2 C library

### orbit2-bt

The orbit2 build tools, to enable building:

- An orbit2 based service common/client side based on the orbit2-sys library, build from an idl. This should be fully automated as there is NO code to implenment

- An orbit2 based service implementation. Based on the common. This should provide some tooling to make it easier to implement the service inside your server application.