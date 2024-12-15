# Creating modules and using them
In root project directory create 2 modules:
- hello/mod.rs
- variables/mod.rs

`hello/mod.rs`
```rust
pub fn main() {  
    println!("hello!")  
}
```

`variables/mod.rs`
```rust
pub fn main(){  
    println!("variables!");  
}
```

`main.rs`
```rust
mod hello;  
mod variables;  
  
fn main() {  
    hello::main();  
    variables::main();  
}
```

## Explanation

- By default rust searches in all `mod.rs` files. The directory it's in declares the module name.
- with `mod.rs` the module code does not need know its a module
- functions have to be declared as `pub`, no declaration defaults to `private`