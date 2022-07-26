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

```lisp
> (defn hello [a] 3)
#<Function:hello>
> (hello 10)
3
```

```lisp
> (if true 1 0)
1
> (if false 1 0)
0
```

```sh
# Running a program from a file
$ cat example.rsp
(defn add-two [a] (add a 2))

(define a 123)
(println (add-two 1))
(println (add-two a))
$ cargo run -- example.rsp 2>/dev/null
3
125
```
