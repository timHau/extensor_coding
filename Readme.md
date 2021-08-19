# Extensor Coding

## Switch implementations

There are different implementations for the ExTensor and the Matrix structure. You switch
between them by simple editing the `Cargo.toml` file.
Just edit the `[features]` default value. If you for example want to use the bitvec implementation
of the extensor and the sparse hash implementation of the matrix use

```
[features]
default = ["extensor_bitvec", "matrix_sparse_hash"]
```

Options are:

- ExTensor:
  - `extensor_bitvec` - Basis as bitvector
  - `dense_hashmap` - Basis as HashMap
- Matrix
  - `naive` - as 1-D vec
  - `sparse_hash` - as sparse HashMap
  - `sparse_triples` - as sparse vec of triples (row, col, value)

## Run

Run the release Version via

```
cargo run --release
```

## Run Tests

To run the tests simply do a

```
cargo test
```

If you only want to run tests in a certain module e.g. all tests in the graph module do

```
cargo test -- graph::tests
```

## Run Benchmarks

All Benchmarks are located under the `benches/` directory. You can execute them with

```
cargo bench
```

## Create Docs

Comments can be turned into a documentation with

```
cargo doc --open --no-deps
```
