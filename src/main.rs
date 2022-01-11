use std::alloc::{alloc_zeroed, Layout};
use std::env;
use std::process;
use sysinfo::SystemExt;

fn allocate_memory(chunk_size: usize, total: i32, pointers: &mut Vec<*mut u8>) {
    unsafe {
        for _ in 0..total {
            let layout = Layout::from_size_align(chunk_size,1).unwrap();
            let pointer = alloc_zeroed(layout);
            pointers.push(pointer);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: first argument must be amount of memory to allocate.");
        process::exit(1);
    }
    let allocation_amount: i32 = args[1].parse().unwrap();

    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    if allocation_amount < 1 || allocation_amount as u64 > system.available_memory()/1024 {
        eprintln!("Error: amount of memory must be between 1 and {}.", system.available_memory()/1024);
        process::exit(1);
    }

    let mut pointers: Vec<*mut u8> = Vec::new();
    let chunk_size = 1_048_576;
    println!("total memory: {} MB, available memory: {} MB, amount to be allocated: {} MB", system.total_memory()/1024, system.available_memory()/1024, allocation_amount);

    println!("Press enter to allocate.");
    let mut key = String::new();
    let _input = std::io::stdin().read_line(&mut key).unwrap();

    allocate_memory(chunk_size, allocation_amount, &mut pointers);

    println!("Press enter to stop and deallocate");
    let mut key = String::new();
    let _input = std::io::stdin().read_line(&mut key).unwrap();
}
