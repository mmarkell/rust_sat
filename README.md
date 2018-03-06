# rust_sat

## my first rust project...implemented a basic DPLL SAT solver

Please pardon my poor Rust style, as this is my first project and I definitely am still learning how to write clean, simple Rust code.

Rust is a language that is both one of the "hardest" languages I've used as well as one of the most beautiful. I am a huge fan of the
principles Rust follows regarding memory safety, even if there is a learning curve to it. Iterators feel as natural as python, it is as 
low level as C, but nearly as safe and clean as a pure functional language. 

## If you want to run

```
clone
cargo build
cargo run
```

## testing
you can write tests to stdin when you run the program with the CNF format:
```
1
2 -3
```
could be read as 
```
(a) and
(b or not c)
```
