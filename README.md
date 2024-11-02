simple parser for amd64 elf files based on

  https://refspecs.linuxfoundation.org/


basic usage:

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
