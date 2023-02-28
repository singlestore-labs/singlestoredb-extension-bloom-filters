# SingleStoreDB extension template in Rust

![Rust Build](https://github.com/singlestore-labs/singlestoredb-extension-rust-template/actions/workflows/rust-docker.yml/badge.svg) ![Release](https://github.com/singlestore-labs/singlestoredb-extension-rust-template/actions/workflows/release.yml/badge.svg)

This repository provides a template repository for developing Wasm UDFs and TVFs for [SingleStoreDB](https://www.singlestore.com/) written in Rust. It contains a [VS Code](https://www.singlestore.com/) development container to help users get bootstrapped quickly, but can be used in any IDE that supports Rust development.

## Tools

To use the tools in this repo, you will need to have Docker installed on your system.  Most of these tools can be installed locally as well:

### [rust/cargo](https://www.rust-lang.org)
The Rust compiler with the *stable* toolchain.  It has been configured with compilation targets for the default platform: *wasm32-wasi*.

### [WIT bindgen](https://github.com/WebAssembly/wasi-sdk)
A language binding generator for the WIT IDL.

## Useful other tools

### [writ](https://github.com/singlestore-labs/writ)
A utility to help test Wasm functions locally without the need to create a separate driver program.  Please see its dedicated [Git Repository](https://github.com/singlestore-labs/writ) for more information.

### [pushwasm](https://github.com/singlestore-labs/pushwasm)
A utility that allows you to easily import your locally-built Wasm function into SingleStoreDB as a UDF or TVF.  Please see its dedicated [Git Repository](https://github.com/singlestore-labs/pushwasm) for more information.

## Development

The project provides a simple rust project which automatically generates the rust bindings & compiles the sources into a Wasm binary. Afterwards, a utility script is available to generate a SQL import script to load the UDFs/TVFs into a SingleStore instance.

Alternatively each step can be done individually using the following workflow:

1. Start with the module interface, as defined in the [extension.wit](https://github.com/singlestore-labs/singlestoredb-extension-rust-template/blob/main/extension.wit) file. Using the `wit-bindgen` tool you can generate the C stubs required to call the newly defined Wasm functions: 
    ```sh
    wit-bindgen c --export extension.wit
    ```

1. Compile your program using the rust compiler targeting the `wasm32-wasi` toolchain:
    ```sh
    # Debug build
    cargo wasi build
    # Release build
    cargo wasi build --release
    ```

1. Once the Wasm binary has been created, you can create a SQL import statement using the [create_loader.sh](https://github.com/singlestore-labs/singlestoredb-extension-rust-template/blob/main/create_loader.sh) script. This creates a `load_extension.sql` file importing the TVF/UDFs using Base64:
    ```sh
    mysql -h <my-host> -u <my-yser> -p -D <my-database> < load_extension.sql
    ```
    Alternatively you can use the `pushwasm` tool to push a single UFD/TVF:
    ```sh
    pushwasm --tvf --prompt mysql://<my-user>@<my-host>:3306/<my-database> --wit extension.wit extension.wasm greet
    ```

## Additional Information

To learn about the process of developing a Wasm UDF or TVF in more detail, please have a look at our [tutorial](https://singlestore-labs.github.io/singlestore-wasm-toolkit/html/Tutorial-Overview.html).

The SingleStoreDB Wasm UDF/TVF documentation is [here](https://docs.singlestore.com/managed-service/en/reference/code-engine---powered-by-wasm.html).

## Resources

* [Documentation](https://docs.singlestore.com)
* [Twitter](https://twitter.com/SingleStoreDevs)
* [SingleStore forums](https://www.singlestore.com/forum)
* [SingleStoreDB extension template for C++](https://github.com/singlestore-labs/singlestoredb-extension-cpp-template)

