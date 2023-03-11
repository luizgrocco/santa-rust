use rand::Rng;
use std::fmt;

pub struct Elf {
    state: ElfState,
    keep_going: bool,
    id: usize,
}
#[derive(fmt::Debug)]
pub enum ElfState {
    Working,
    Trouble,
    AtSantasDoor,
}

impl Elf {
    pub fn new(id: usize) -> Elf {
        Elf {
            id,
            state: ElfState::Working,
            keep_going: true,
        }
    }

    pub fn terminate(&mut self) {
        self.keep_going = false;
    }

    pub fn help(&mut self) {
        self.state = ElfState::Working;
    }

    pub fn work(&mut self) {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(1..=100);
        if num < 5 {
            self.state = ElfState::Trouble;
        }
    }

    pub fn report(self) {
        println!("Elf {} : {:?}", self.id, self.state);
    }
}
