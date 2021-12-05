# PGR

A playground for experiments with Rust inside PG extension.

## How to install it?

```
cd pg
make install
```

## How to use it?

```sql
create extension pgr;
select hello_rust();
```

This example calls a Rust function `hello_world()` inside PG that:

1. Creates a C compatible string with safe Rust.
1. Allocates a memory chunk using PostgreSQL allocator with unsafe Rust.
1. Copies the string from the Rust memory to the palloced one and attaches the pointer and some additional information to `PgMemoryChunk` structure.
1. Returns `PgMemoryChunk` (it has a C ABI) to PG function, retrieves data and prints the string.

We should not care about Rust memory as its compiler automatically adds deallocations when the objects runs out of the scope.
