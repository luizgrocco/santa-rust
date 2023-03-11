mod christmas;

use crate::christmas::elf::Elf;

fn main() {
    let mut my_elf = Elf::new(0);
    my_elf.help();
}
