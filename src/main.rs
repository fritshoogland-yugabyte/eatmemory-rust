extern crate libc;
use std::process;
use sysinfo::SystemExt;
use libc::{c_void, mlock};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long)]
    /// if set displays memory and swap details.
    query: bool,
    #[structopt(short, long)]
    /// if set displays memory and swap details plus actions.
    verbosity: bool,
    #[structopt(short, long)]
    /// if set try to call mlock()
    memlock: bool,
    #[structopt(short, long, default_value = "0")]
    /// size of allocation in MB, megabytes.
    size_definition: i64,
    #[structopt(short, long, default_value = "-1")]
    /// size of memory (paged in) in MB, megabytes. -1 means equal to size_definition.
    alloc_definition: i64,
}

fn print_memory() {
    let system = sysinfo::System::new_all();
    println!("{:20}: {:9} MB\n{:20}: {:9} MB\n{:20}: {:9} MB\n{:20}: {:9} MB",
             "total memory", system.total_memory()/1024,
             "available memory", system.available_memory()/1024,
             "free memory", system.free_memory()/1024,
             "used memory", system.used_memory()/1024
            );
    println!("{:20}: {:9} MB\n{:20}: {:9} MB\n{:20}: {:9} MB",
             "total swap", system.total_swap()/1024,
             "free swap", system.free_swap()/1024,
             "used swap", system.used_swap()/1024
            );
}

fn main() {
    let options = Opts::from_args();
    let query = options.query as bool;
    let memlock = options.memlock as bool;
    let verbosity = options.verbosity as bool;
    let size_definition = options.size_definition as i64;
    let allocation_definition = options.alloc_definition as i64;

    if query {
        print_memory();
        process::exit(0);
    };
    if size_definition == 0 {
        let mut app  = Opts::clap();
        app.print_help().unwrap();
        println!();
        process::exit(0);
    }
    if verbosity { print_memory() }

    if verbosity { println!("creating vec with size {} MB", size_definition) }
    let size_as_u8: usize = (size_definition*1024*1024).try_into().unwrap();
    let allocation_as_u8: usize = if allocation_definition == -1 {
        size_as_u8
    } else {
        (allocation_definition*1024*1024).try_into().unwrap()
    };
    let mut hog = vec![0u8; size_as_u8];

    if memlock {
        if verbosity { println!("executing mlock") }
        let ptr = hog.as_ptr() as *const c_void;
        let result = unsafe { mlock(ptr, size_as_u8) };
        if result != 0 { println!("mlock did not succeed, returned: {}. hint: look at limits via ulimit.", result) }
    }

    if verbosity { println!("allocating vec for {} MB", allocation_definition) }
    hog[0..allocation_as_u8].fill(0);

    if verbosity { print_memory() }

    println!("done. press enter to stop and deallocate");
    let mut key = String::new();
    let _input = std::io::stdin().read_line(&mut key).unwrap();
}
