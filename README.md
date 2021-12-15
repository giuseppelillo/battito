# Battito

## Modules

### battito-lib

The core of the project. It exposes the `transform` function that's able to parse the input and generate a pattern representation.
More info here: https://docs.google.com/presentation/d/1RfbiHeF0CticCYikbV3Q2uXpJl1fMQmfv3qBRHtvolA

### battito-app

A simple application that reads standard input and runs the library

### battito-max

Contains the C bindings for using the library inside a Max external.


## How to run it

```
cargo run -- --subdivision $SUBDIVISION
```

E.g.

```
cargo run -- --subdivision 1920
```
