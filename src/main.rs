#[macro_use]
extern crate serde_derive;

mod punkfile;
mod isa;
mod memory;

use punkfile::punk_file::PunkFile;

fn main() {
    println!("Hello, world!");

    let pk = PunkFile::from_file("scheme.json");
    println!("{:?}", pk)
}
