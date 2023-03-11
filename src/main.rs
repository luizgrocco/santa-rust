mod christmas;

use crate::christmas::elf::Elf;

fn main() {
    let mut my_elf = Elf::new(0);
    let borrowedElf1 = &my_elf;
    let borrowedElf2 = &my_elf;

    let elves: Vec<Elf> = (1..=2).map(|id| Elf::new(id)).collect::<Vec<_>>();

    // let handles = core_ids
    //     .into_iter()
    //     .map(|core_id| {
    //         let counter = Arc::clone(&counter);
    //         thread::spawn(move || {
    //             // Pin this thread to a single CPU core.
    //             let res = set_for_current(core_id);
    //             if res {
    //                 println!("Cpu {} successfully set affinity", core_id.id);
    //                 let mut count = counter.lock().unwrap();
    //                 let lowerbound = MAX_SIZE * core_id.id as i64;
    //                 let upperbound = core_id.id as i64 * MAX_SIZE + MAX_SIZE;
    //                 println!("lowerbound: {}, upperbound: {}", lowerbound, upperbound);
    //                 for i in lowerbound..upperbound {
    //                     *count += i;
    //                 }
    //             }
    //         })
    //     })
    //     .collect::<Vec<_>>();

    my_elf.help();
    my_elf.work();
}
