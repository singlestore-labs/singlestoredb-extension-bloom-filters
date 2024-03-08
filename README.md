# Bloom filters in SingleStoreDB

![Rust Build](https://github.com/singlestore-labs/singlestoredb-extension-bloom-filters/actions/workflows/rust-docker.yml/badge.svg) ![Release](https://github.com/singlestore-labs/singlestoredb-extension-bloom-filters/actions/workflows/release.yml/badge.svg)

**Attention**: The code in this repository is intended for experimental use only and is not fully tested, documented, or supported by SingleStore. Visit the [SingleStore Forums](https://www.singlestore.com/forum/) to ask questions about this repository.

## Introduction

[Bloom filters](https://hur.st/bloomfilter/?n=1000000&p=0.01&m=&k=100) are a space-efficient probabilistic data structure that can be used to test whether an 
element is a member of a set. They are commonly used in databases to test whether a value is present in a column,
without having to scan the entire column. Bloom filters are also used in distributed systems to test whether a value
is present in a distributed set, without having to communicate with all nodes in the system.

Bloom filters are an efficient way to answer the question "Is this value POTENTIALLY present in this set?". 
It determines whether the element either definitely is not in the set or may be in the set.

## Contents
This library provides the following database objects.

### `bloom_filter`
This is a User-Defined Aggregate (UDAF) that will generate a bloom filter from a column of strings.

### `bloom_maybe_exists`
This is a User-Defined Function (UDF) that will always returns 0 if its argument string does not match the filter.  If the string does match the filter, then it will *usually* return 1, but may 0.

## Building
The Wasm module can be built using the following commands.  The build requires Rust with the WASI extension.
```bash
# Install the WASI cargo extension.
cargo install cargo-wasi

# Compile the Wasm module.
cargo wasi build --release
```
The binary will be placed in `target/wasm32-wasi/release/extension.wasm`.

## Deployment to SingleStoreDB

To install these functions using the MySQL client, use the following commands.  This command assumes you have built the Wasm module and your current directory is the root of this Git repo.  Replace `$DBUSER`, `$DBHOST`, `$DBPORT`, and `$DBNAME` with, respectively, your database username, hostname, port, and the name of the database where you want to deploy the functions.
```bash
cat <<EOF | mysql -u $DBUSER -h $DBHOST -P $DBPORT -D $DBNAME -p
CREATE OR REPLACE AGGREGATE bloom_filter(
    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL)
RETURNS LONGBLOB NOT NULL
WITH STATE HANDLE
AS WASM FROM BASE64 '`base64 -w 0 target/wasm32-wasi/release/extension.wasm`'
WITH WIT FROM BASE64 '`base64 -w 0 extension.wit`'
INITIALIZE WITH bloom_init_handle
ITERATE WITH bloom_update_handle
MERGE WITH bloom_merge_handle
TERMINATE WITH bloom_serialize_handle
SERIALIZE WITH bloom_serialize_handle
DESERIALIZE WITH bloom_deserialize_handle
EOF

cat <<EOF | mysql -u $DBUSER -h $DBHOST -P $DBPORT -D $DBNAME -p
CREATE OR REPLACE FUNCTION bloom_maybe_exists
AS WASM FROM BASE64 '`base64 -w 0 target/wasm32-wasi/release/extension.wasm`'
WITH WIT FROM BASE64 '`base64 -w 0 extension.wit`'
EOF
```

Alternatively, you can install these functions using [pushwasm](https://github.com/singlestore-labs/pushwasm) with the following command lines.  As above, be sure to substitute the environment variables with values of your own.
```bash
pushwasm udaf --force --prompt --name bloom_filter \
    --wasm target/wasm32-wasi/release/extension.wasm \
    --wit extension.wit \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME" \
    --abi canonical \
    --type 'longblob not null' \
    --arg 'text character set utf8mb4 collate utf8mb4_general_ci not null' \
    --state HANDLE \
    --init bloom_init_handle \
    --iter bloom_update_handle \
    --merge bloom_merge_handle \
    --terminate bloom_serialize_handle \
    --serialize bloom_serialize_handle \
    --deserialize bloom_deserialize_handle

pushwasm udf --force --prompt --name bloom_maybe_exists \
    --wasm target/wasm32-wasi/debug/extension.wasm \
    --wit extension.wit \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
```

## Usage
The following is a simple example that creates two tables with a columns of strings.  The first table's column is used to generate a Bloom Filter, which we store in a User Defined Variable.  We then run the Bloom Filter on the strings in the second table.

```sql
CREATE TABLE t1(s TEXT);
INSERT INTO t1(s) VALUES ("Bob"), ("Frank"), ("George"), ("Henry");
CREATE TABLE t2(s TEXT);
INSERT INTO t2(s) VALUES ("Jake"), ("Martin"), ("Flo"), ("Frank"), ("George");

SELECT bloom_filter(s) FROM t1 INTO @bloom;

SELECT s, bloom_maybe_exists(@bloom, s) FROM t2;
```
This should produce the following output:
```console
+--------+-------------------------------+
| s      | bloom_maybe_exists(@bloom, s) |
+--------+-------------------------------+
| Jake   |                             0 |
| Flo    |                             0 |
| George |                             1 |
| Martin |                             0 |
| Frank  |                             1 |
+--------+-------------------------------+
5 rows in set (0.610 sec)
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

