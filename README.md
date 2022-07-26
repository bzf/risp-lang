risp-lang
=========

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

```scheme
> (add 1 2 3)
6
```

```scheme
> (subtract 3 2)
1
```

```scheme
> (define foo (add 5 5))
10
> (add foo 5)
15
```

```scheme
> (defn hello [a] 3)
#<Function:hello>
> (hello 10)
3
> (defn count [a]
    (if (is-empty a) 0 (add (count (cdr a)) 1)))
#<Function:hello>
> (count (list))
0
> (count (list 1 2 3))
3
```

```scheme
> (if true 1 0)
1
> (if false 1 0)
0
```

```scheme
> (is-nil 1)
false
> (is-nil nil)
true
```

```scheme
> (list 1 2 3)
(1 2 3)
> (define foo (list 1 2 3))
(1 2 3)
> (append foo 1)
(1 2 3 1)
> (prepend foo 1)
(1 1 2 3)
> (car foo)
1
> (cdr foo)
(2 3)
> (is-empty foo)
false
> (is-empty (list))
true
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

# Implementing count and reverse
$ cat example.rsp
(define my-list (list 1 2 3))
(println "my-list =" my-list)

(defn count [a]
  (if (is-empty a) 0 (add (count (cdr a)) 1)))

(println "count:" (count my-list))

(defn reverse [a]
  (if (is-empty a) (list) (append (reverse (cdr a)) (car a))))

(println "reverse:" (reverse my-list))
$ cargo run -- example.rsp 2>/dev/null
my-list = (1 2 3)
count: 3
reverse: (3 2 1)
```
