risp
====

A Lisp interpreter written in Rust for fun and giggles.

## Building it

```sh
cargo build
```

## Running the REPL

```sh
cargo run
```

## Examples

```lisp
> (add 1 2 3)
6
```

```lisp
> (subtract 3 2)
1
```

```lisp
> (define foo (add 5 5))
10
> (add foo 5)
15
```
