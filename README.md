# ark-elliptic-curve-groups

A Rust library that provides wrappers around arkworks elliptic curve groups to implement the traits from the `elliptic-curve` crate. This enables interoperability between arkworks curve implementations and libraries that rely on the `elliptic-curve` traits.

## Features

- Wraps arkworks elliptic curve groups to implement:
  - `elliptic-curve::Group`
  - `group::GroupEncoding` (with compressed point encoding)
- Wraps arkworks field elements to implement `ff::Field`
- Supports a wide variety of curves from the arkworks ecosystem:
  - BLS12-377
  - BLS12-381
  - BN254
  - BW6-761
  - BW6-767
  - CP6-782
  - MNT4-298
  - MNT6-298
  - MNT4-753
  - MNT6-753
  - Edwards curves (Ed25519, Ed448, etc.)
  - Twisted Edwards curves
  - Short Weierstrass curves

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ark-elliptic-groups = { git = "https://github.com/tangle-network/ark-elliptic-curve-groups" }
```

### Basic Example

```rust
use ark_elliptic_groups::{ArkGroupWrapper, ArkScalarWrapper};
use ark_bls12_381::{G1Projective, Fr};
use elliptic_curve::Group;
use group::GroupEncoding;

// Wrap an arkworks group element
let point = G1Projective::generator();
let wrapped_point = ArkGroupWrapper::new(point);

// Wrap an arkworks scalar
let scalar = Fr::from(42u64);
let wrapped_scalar = ArkScalarWrapper::new(scalar);

// Use with elliptic-curve traits
let result = wrapped_point * wrapped_scalar;

// Serialize/deserialize points
let bytes = wrapped_point.to_bytes();
let decoded = ArkGroupWrapper::<G1Projective>::from_bytes(&bytes);
```

### Implementing Generic Protocols

The wrappers allow you to write generic code that works with both native `elliptic-curve` types and arkworks types:

```rust
use elliptic_curve::Group;
use group::GroupEncoding;

fn double_and_serialize<G: Group + GroupEncoding>(point: G) -> G::Repr {
    let doubled = point + point;
    doubled.to_bytes()
}

// Works with wrapped arkworks groups
let encoded = double_and_serialize(wrapped_point);
```

## Implementation Details

The library provides two main wrapper types:

- `ArkGroupWrapper<G>`: Wraps an arkworks curve group
  - Implements standard group operations
  - Provides compressed point encoding via `GroupEncoding`
- `ArkScalarWrapper<F>`: Wraps an arkworks field element

These wrappers implement the necessary traits to make them compatible with the `elliptic-curve` ecosystem while maintaining the performance characteristics of the underlying arkworks implementations.

## Build-time Constants

The library uses build scripts to generate necessary constants for each supported curve, ensuring optimal performance without runtime overhead.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
