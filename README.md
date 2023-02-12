GOLD Parser Engine in RUST
==========================

This crate provides an engine that can read a compiled [Enhanced Grammar Table](http://goldparser.org/doc/egt/index.htm) created with the [GOLD Parsing System](http://goldparser.org/index.htm) and generate a skeleton parser in rust for your custom language.

```rust
use goldparser-rs {
    engine::Builder,
    parser::{GOLDParser},
    vm::RuleHandler,
}

fn main() {
    let parser = Builder::from("mylang.egt");
    parser.load_source("test.src");
    if let Ok(ast) = parser.parse() {
        println!("{}",ast);
    } else {
        println!("Problems parsing");
    }

}
```

>For more information on how it works, see the [documentation](https://droidengineer.github.io/goldparser-rs/) or the [Wiki](https://github.com/droidengineer/goldparser-rs/wiki).

Contents:
---------

1. [How To...](#how-to)
    1. [Include in your project](#include-in-your-project)
    2. [Use](#use)
5. [Binaries](#binaries)
    1. [Cargo Install](#cargo-install)
    2. [Github](#github)

How To...
---------

## Include in your project

You can install the crate for use by including this in your _Cargo.toml_:
```shell
    [dependencies]
    goldparser-rs = "0.1"
```

## Use

...<TODO>...



## Binaries

`egtutil` is a binary in the \bin directory for basic operations on compiled `Enhanced Grammar Tables` and serves as a working example of implementing the parsers from this crate. The 
interactive feature implements a [REPL](https://www.digitalocean.com/community/tutorials/what-is-repl)-like environment to walk through the [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) of your parsed source code.

### Cargo Install

Install the `goldparser-rs` crate to get the `egtutil` binary.

```
cargo install goldparser-rs
```

### Github

Alternatively, you can grab it on github and make it yourself.

```
git clone https://github.com/droidengineer/goldparser-rs.git
cd goldparser-rs
cargo build
```




