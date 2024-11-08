[![Crates.io](https://img.shields.io/crates/v/simple_elf.svg)](https://crates.io/crates/simple_elf)
[![Docs.rs](https://docs.rs/simple_elf/badge.svg)](https://docs.rs/simple_elf)

# simple_elf

simple parser for amd64 elf files

## ℹ️ Overview

this is simple elf parser based on elf64 specification its intended for educational puposes.
it parses the elf file to an elf struct in rust.

the elf type has three fields:
 - elf header
 - sections: list of section objects corresponding to sections inside the file
 - programs: list of programs corresponding to program headers inside the file

## features:

 - view headers like file header, section headers, program headers
 - view section data including symbols, relocations, strings, ...
 - view raw binary data of sections

### 🚀 basic usage:

```Rust

use std::fs;
use simple_elf::Elf64;

fn main() {
    let raw = fs::read("/usr/bin/yes").unwrap();
    let elf = Elf64::from(&raw);
    println!("{:?}", elf.header);
    for section in elf.sections {
        println!("{}", section.name);
    }
}

```

## 📖 further readings

 - https://refspecs.linuxfoundation.org/
    it includes elf64 specification and x86 abi document that are used to implement this crate

## 🤝 Contributing Guidelines
 any contribution are wellcome
