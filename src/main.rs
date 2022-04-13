extern crate libc;
use std::process;
use sysinfo::SystemExt;
use libc::{c_void, mlock};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long)]
    /// if set displays memory and swap details, then quits.
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
    #[structopt(short, long)]
    /// size of memory (paged in) in MB, megabytes. By default, the entire size is allocated.
    alloc_definition: Option<i64>,
}

// this function shows the memory usage via the crate 'sysinfo'.
fn print_memory() {
    let system = sysinfo::System::new_all();
    println!("{:20}: {:9} MB",
             "available memory", system.available_memory()/1024
    );
    println!("{:20}: {:9} MB, {:20}: {:9} MB, {:20}: {:9} MB",
             "total memory", system.total_memory()/1024,
             "free memory", system.free_memory()/1024,
             "used memory", system.used_memory()/1024
    );
    println!("{:20}: {:9} MB, {:20}: {:9} MB, {:20}: {:9} MB",
             "total swap", system.total_swap()/1024,
             "free swap", system.free_swap()/1024,
             "used swap", system.used_swap()/1024
    );
}

fn main() {
    // obtain CLI options and set variables.
    let options = Opts::from_args();
    let query = options.query as bool;
    let memlock = options.memlock as bool;
    let verbosity = options.verbosity as bool;
    let size_definition = options.size_definition as i64;
    let allocation_definition = match options.alloc_definition {
        Some(alloc_definition) => alloc_definition,
        None => size_definition,
    };

    // the query otpion simply shows the memory usage and stops.
    if query {
        print_memory();
        process::exit(0);
    };
    // if there is no memory size specified, we can't allocate anything.
    // therefore, print the usage, and quit.
    if size_definition == 0 {
        let mut app  = Opts::clap();
        app.print_help().unwrap();
        println!();
        process::exit(0);
    }
    //
    if verbosity { print_memory() }

    // here the allocation starts
    if verbosity { println!("creating vec with size {} MB", size_definition) }
    // recalculate the given size in MB to bytes (1024*102), and make it a usize (processor pointer sized unsigned integer).
    let size_as_u8: usize = (size_definition*1024*1024).try_into().unwrap();
    let alloc_as_u8: usize = (allocation_definition*1024*1024).try_into().unwrap();

    // this is the allocation for size, which does allocate, but not page in.
    let mut hog = vec![0u8; size_as_u8];
    // in order to be able to use the allocation, we get a pointer to 'hog'.
    let ptr = hog.as_ptr() as *const c_void;
    //
    if verbosity { println!("pointer to hog: {:?}", ptr) }

    // if memlock is chosen, we perform mlock.
    // mlock is taken directly from libc, hence unsafe.
    // this will fail on most linux systems because of the ulimits set.
    if memlock {
        if verbosity { println!("executing mlock") }
        let result = unsafe { mlock(ptr, size_as_u8) };
        if result != 0 { println!("mlock did not succeed, returned: {}. hint: look at limits via ulimit.", result) }
    }

    //
    if verbosity { println!("allocating vec for {} MB", allocation_definition) }
    // This takes the 'hog' vec, and fills it with zero's.
    // by doing this, it will page in memory.
    hog[0..alloc_as_u8].fill(0);

    // after the allocation and paging, print memory again.
    if verbosity { print_memory() }

    println!("done. press enter to stop and deallocate");
    let mut key = String::new();
    let _input = std::io::stdin().read_line(&mut key).unwrap();
}
