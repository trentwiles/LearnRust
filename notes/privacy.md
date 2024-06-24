## Privacy & Modules

### Declaring a Module

```
mod garden;
```

Rust will search `src/garden.rs` and `src/garden/mod.rs`

If you are in a folder and declare a module, rust will search in the folder's subfolder.

### Paths to Module Code

Using the example from above:
`crate::garden::YourTypeHere`

### Public/Private Modules

You can force a module to be public by adding the `pub` keyword in front of a module declaration.

### Use Keyword

Instead of having to write out `crate::garden::YourTypeHere` multiple times, you can add `use crate::garden::YourTypeHere` to the top of the file, and then call `YourTypeHere`.

### Full Example

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

`src/main.rs` contents:

```
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

* `pub mod garden;` tells compiler to use code from `src/garden.rs`
* `src/garden.rs` contains the following:

```
pub mod vegetables;
```

* Which in turn tells the compiler to include src/garden/vegetables.rs, which contains:

```
#[derive(Debug)]
pub struct Asparagus {}
```

* So when `let plant = Asparagus {};` is called in the main.rs file, the code will compile!

### More Module Grouping