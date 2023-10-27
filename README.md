# Osi-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

## description 

Osi-src crate is a *-src crate. This links [Osi] libraries to executable build by cargo, but does not provide Rust bindings.[Osi] build with [CoinUtils] ([CoinUtils-src]) support.

By this package, you don't need to worry about installing Osi in the system, and it's a package for **all platforms**.

Osi (Open Solver Interface) provides an abstract base class to a generic linear programming (LP) solver, along with derived classes for specific solvers. Many applications may be able to use the Osi to insulate themselves from a specific LP solver. That is, programs written to the OSI standard may be linked to any solver with an OSI interface and should produce correct results. The OSI has been significantly extended compared to its first incarnation. Currently, the OSI supports linear programming solvers and has rudimentary support for integer programming.

## Usage

1. Add the following to your `Cargo.toml`:

```toml
[dependencies]
osi-src = "0.2"
```

2. Add the following to your `lib.rs`:

```toml
extern crate osi_src;
```

This package does not provide bindings. Please use [coincbc-sys], [coinclp-sys] to use Cbc, Clp, e.g.

```toml
[dependencies]
coincbc-sys = { version = "0.2" }
```

## Configuration
The following Cargo features are supported:

* `default` to build `Osi` without any solver support;
* `osicpx` to enable the Cplex support;
* `osiglpk` to enable the GLPK support;
* `osigrb` to enable the Gurobi support;
* `osimsk` to enable the Mosek support;
* `osispx` to enable the Soplex support;
* `osixpr` to enable the XPRESS support;

The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_COINUTILS_STATIC` to link to CoinUtils statically;
* `CARGO_COINUTILS_SYSTEM` to link to CoinUtils system library;
* `CARGO_OSI_STATIC` to link to Osi statically;
* `CARGO_OSI_SYSTEM` to link to Osi system library;

Set the environment variable to `1` to enable the feature. For example, to link to system library dynamically, set `CARGO_${LIB_NAME}_SYSTEM` to `1`; to link to system library statically, set both `CARGO_${LIB_NAME}_SYSTEM` and `CARGO_${LIB_NAME}_STATIC` to `1`.

## Windows and vcpkg

On Windows, if `${LIB_NAME}_SYSTEM` is set to `1`, `osi-src` will use 
[vcpkg] to find Osi. Before building, you must have the correct Osi 
installed for your target triplet and kind of linking. For instance,
to link dynamically for the `x86_64-pc-windows-msvc` toolchain, install
 `osi` for the `x64-windows` triplet:

```sh
vcpkg install osi --triplet x64-windows
```

To link Osi statically, install `osi` for the `x64-windows-static-md` triplet:

```sh
vcpkg install osi --triplet x64-windows-static-md
```

To link Osi and C Runtime (CRT) statically, install `osi` for the `x64-windows-static` triplet:

```sh
vcpkg install osi --triplet x64-windows-static
```

and build with `+crt-static` option

```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) for detail.

## Cross Compilation

you can compile it for the other target by providing the `--target` option to 
`cargo build`. 


| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-linux-androideabi`            | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-gnu`              | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[CoinUtils]: https://github.com/coin-or/CoinUtils
[Osi]: https://github.com/coin-or/Osi

[CoinUtils-src]: https://github.com/Maroon502/coinutils-src
[coincbc-sys]: https://github.com/Maroon502/coincbc-sys
[coinclp-sys]: https://github.com/Maroon502/coinclp-sys

[vcpkg]: https://github.com/Microsoft/vcpkg


[documentation-img]: https://docs.rs/osi-src/badge.svg
[documentation-url]: https://docs.rs/osi-src
[package-img]: https://img.shields.io/crates/v/osi-src.svg
[package-url]: https://crates.io/crates/osi-src
[license-img]: https://img.shields.io/crates/l/osi-src.svg
[license-url]: https://github.com/Maroon502/osi-src/blob/master/LICENSE.md