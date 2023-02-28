# Bloom filters in SingleStoreDB

![Rust Build](https://github.com/singlestore-labs/singlestoredb-extension-bloom-filters/actions/workflows/rust-docker.yml/badge.svg) ![Release](https://github.com/singlestore-labs/singlestoredb-extension-bloom-filters/actions/workflows/release.yml/badge.svg)

## Introduction

[Bloom filters](https://hur.st/bloomfilter/?n=1000000&p=0.01&m=&k=100) are a space-efficient probabilistic data structure that can be used to test whether an 
element is a member of a set. They are commonly used in databases to test whether a value is present in a column,
without having to scan the entire column. Bloom filters are also used in distributed systems to test whether a value
is present in a distributed set, without having to communicate with all nodes in the system.

Bloom filters are an efficient way to answer the question "Is this value POTENTIALLY present in this set?". 
It determines whether the element either definitely is not in the set or may be in the set.
z
## Usage in SingleStore

The library can import the following [UDF](https://docs.singlestore.com/managed-service/en/reference/code-engine---powered-by-wasm/create-wasm-udfs.html):
* `bloom_maybe_exists`: checks if the value is MAYBE part of the set

in addition the aggregate function `bloom_filter` is created as follows:
```sql
CREATE OR REPLACE AGGREGATE bloom_filter(text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL)
RETURNS LONGBLOB NOT NULL
WITH STATE HANDLE
AS WASM FROM BASE64 '$WASM_B64'
WITH WIT FROM BASE64 '$WIT_B64'
INITIALIZE WITH bloom_init_handle
ITERATE WITH bloom_update_handle
MERGE WITH bloom_merge_handle
TERMINATE WITH bloom_serialize_handle
SERIALIZE WITH bloom_serialize_handle
DESERIALIZE WITH bloom_deserialize_handle;
```

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

* [Bloom filters](https://hur.st/bloomfilter/?n=1000000&p=0.01&m=&k=100)
* [Documentation](https://docs.singlestore.com)
* [Twitter](https://twitter.com/SingleStoreDevs)
* [SingleStore forums](https://www.singlestore.com/forum)
* [SingleStoreDB extension template for C++](https://github.com/singlestore-labs/singlestoredb-extension-cpp-template)

