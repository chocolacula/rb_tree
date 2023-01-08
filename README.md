# Red Black Tree in Rust

It is an educational project, made with one purpose - teach students algorithms and Rust🦀.  

It covers basic Rust topics like borrowing and includes:

- `cargo` commands
- project structure
- testing
- standard modules like `Option<>` and `Result<>`
- interior mutability and `Rc<RefCell>`
- `unsafe` code

The code is not optimal, it has written in easy to read manner instead.  

A few notes about implementation:

- `RbTree<T>` has only `key` which is the `value` in the same time
- implements `print()` for rendering tree structure in console
- implements `is_valid()` for checking rules violation
- nevertheless has few optimization:
  - build path during traversal instead of store pointer to parent
  - store children in small array to reduce branching

Moreover it contains good comments which covers rotation and colorization.

If you are not even one of my students but have troubles with understanding,  
feel free to contact me for more information and explanation.