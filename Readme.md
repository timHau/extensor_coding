# Extensor Coding

## switch implementations

There are different implementations for the ExTensor and the Matrix structure. You switch 
between them by simple editing the `Cargo.toml` file.
Just edit the `[features]` default value. If you for example want to use the bitvec implementation
of the extensor and the sparse hash implementation of the matrix use

```
[features]
default = ["extensor_bitvec", "matrix_sparse_hash"]
```

Options are: 
* ExTensor:
    * `extensor_bitvec` - Basis as bitvector
    * `dense_hashmap` - Basis as HashMap
* Matrix
    * `naive` - as 1-D vec
    * `naive_parallel` - (very) naive parallelization attempt
    * `sparse_hash` - as sparse HashMap
    * `sparse_triples` - as sparse vec of triples (row, col, value)