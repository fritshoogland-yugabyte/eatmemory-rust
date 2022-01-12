use std::env;
use std::process;
use sysinfo::SystemExt;

fn print_memory() {
    let system = sysinfo::System::new_all();
    println!("total memory: {} MB, available memory: {} MB", system.total_memory()/1024, system.available_memory()/1024);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: first argument must be amount of memory to allocate.");
        process::exit(1);
    }
    let allocation_amount: i32 = args[1].parse().unwrap();

    let system = sysinfo::System::new_all();
    if allocation_amount < 1 || allocation_amount as u64 > system.available_memory()/1024 {
        eprintln!("Error: amount of memory must be between 1 and {}.", system.available_memory()/1024);
        process::exit(1);
    }

    print_memory();

    println!("Press enter to allocate.");
    let mut key = String::new();
    let _input = std::io::stdin().read_line(&mut key).unwrap();

    let amount_as_u8: usize = (allocation_amount*1024*1024).try_into().unwrap();
    let mut hog = vec![0u8; amount_as_u8];
    hog[0..amount_as_u8].fill(0);

    print_memory();
    println!("Press enter to stop and deallocate");
    let mut key = String::new();
    let _input = std::io::stdin().read_line(&mut key).unwrap();
}
