#[macro_use]
extern crate serde_derive;


mod punkfile;

use punkfile::punk_file::PunkFile;

fn main() {
    println!("Hello, world!");

    PunkFile::from_file("test.json");
}
