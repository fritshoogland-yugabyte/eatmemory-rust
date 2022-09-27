extern crate libc;
use std::process;
use sysinfo::SystemExt;
use libc::{c_void, mlock, malloc, memset, free};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long)]
    /// if set displays memory and swap details, then quits.
    query: bool,
    #[structopt(short, long)]
    /// if set displays memory and swap details plus actions.
    verbosity: bool,
    #[structopt(short, long, default_value = "0")]
    /// size of allocation in MB, megabytes.
    init_size_mb: i64,
    #[structopt(short, long)]
    /// size of memory (paged in) in MB, megabytes. By default, the entire size is allocated.
    use_size_mb: Option<i64>,
    #[structopt(short, long, default_value = "native", possible_values(&["malloc", "native", "mlock"]))]
    /// type of allocation
    alloc_type: String,
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

fn wait_for_enter() {
    println!("done. press enter to continue");
    let mut key = String::new();
    let _input = std::io::stdin().read_line(&mut key).unwrap();
}

fn main() {
    // obtain CLI options and set variables.
    let options = Opts::from_args();
    //let query = options.query as bool;
    //let memlock = options.memlock as bool;
    //let verbosity = options.verbosity as bool;
    //let size_definition = options.size_definition as i64;
    let use_size_mb = match options.use_size_mb {
        Some(use_size_mb) => use_size_mb,
        None => options.init_size_mb,
    };

    // the query option simply shows the memory usage and stops.
    if options.query {
        print_memory();
        process::exit(0);
    };
    // if there is no memory size specified, we can't allocate anything.
    // therefore, print the usage, and quit.
    if options.init_size_mb == 0 {
        let mut app  = Opts::clap();
        app.print_help().unwrap();
        println!();
        process::exit(0);
    }
    //
    if options.verbosity { print_memory() }

    // here the allocation starts
    //if options.verbosity { println!("creating vec with size {} MB", options.size_definition) }
    // recalculate the given size in MB to bytes (1024*102), and make it a usize (processor pointer sized unsigned integer).
    let init_size_as_u8: usize = (options.init_size_mb*1024*1024).try_into().unwrap();
    let use_size_as_u8: usize = (use_size_mb*1024*1024).try_into().unwrap();
    println!("init_size: {}, use_size: {}", &init_size_as_u8, &use_size_as_u8);
    match options.alloc_type.as_str() {
        "native" => {
            println!("native mode");
            let mut hog = vec![0u8; init_size_as_u8];
            let ptr = hog.as_ptr() as *const c_void;
            if options.verbosity { println!("1. initialisation. pointer to vec: {:?}", ptr) }
            wait_for_enter();
            if options.verbosity { println!("allocating vec for {} MB", init_size_as_u8/1024/1024) }
            hog[0..use_size_as_u8].fill(0);
            if options.verbosity { print_memory() };
            if options.verbosity { println!("2. page in vec") }
            wait_for_enter();
        },
        "mlock" => {
            println!("mlock mode (uses native mode/vector)");
            let hog = vec![0u8; init_size_as_u8];
            let ptr = hog.as_ptr() as *const c_void;
            if options.verbosity { println!("1. initialisation. pointer to vec: {:?}", ptr) }
            let result = unsafe { mlock(ptr, init_size_as_u8) };
            if result != 0 { println!("mlock did not succeed, returned: {}. hint: look at limits via ulimit.", result) }
            if options.verbosity { print_memory() };
            if options.verbosity { println!("2. mlock vec") }
            wait_for_enter();
        },
        "malloc" => {
            println!("malloc mode");
            let pointer = unsafe{ malloc(init_size_as_u8) };
            if pointer.is_null() {
                println!("malloc did not succeed, returned nullpointer.");
            } else {
                println!("malloc succeeded, pointer: {:?}", pointer);
            };
            if options.verbosity { println!("1. initialisation. pointer returned by malloc {:?}", pointer) }
            wait_for_enter();
            let memset_pointer = unsafe { memset(pointer, 1, use_size_as_u8) };
            if memset_pointer.is_null() {
                println!("memset did not succeed, returned nullpointer.");
            } else {
                println!("memset succeeded, pointer: {:?}", memset_pointer);
            };
            if options.verbosity { println!("2. page in. pointer returned by memset {:?}", memset_pointer) }
            if options.verbosity { print_memory() };
            wait_for_enter();
            println!("calling free");
            unsafe { free(pointer) };
        }
        &_ => unreachable!()
    }

    /*
    // this is the allocation for size, which does allocate, but not page in.
    let mut hog = vec![0u8; size_as_u8];
    // in order to be able to use the allocation, we get a pointer to 'hog'.
    let ptr = hog.as_ptr() as *const c_void;
    //
    if options.verbosity { println!("pointer to hog: {:?}", ptr) }
     */

    /*
    // if memlock is chosen, we perform mlock.
    // mlock is taken directly from libc, hence unsafe.
    // this will fail on most linux systems because of the ulimits set.
    if options.memlock {
        if options.verbosity { println!("executing mlock") }
        let result = unsafe { mlock(ptr, size_as_u8) };
        if result != 0 { println!("mlock did not succeed, returned: {}. hint: look at limits via ulimit.", result) }
    }

     */

    /*
    //
    if options.verbosity { println!("allocating vec for {} MB", allocation_definition) }
    // This takes the 'hog' vec, and fills it with zero's.
    // by doing this, it will page in memory.
    hog[0..alloc_as_u8].fill(0);

    // after the allocation and paging, print memory again.
    if options.verbosity { print_memory() }
     */

    /*
    println!("done. press enter to stop and deallocate");
    let mut key = String::new();
    let _input = std::io::stdin().read_line(&mut key).unwrap();
     */
    //wait_for_enter();
    if options.verbosity { print_memory() }
    println!("3. freed");
    wait_for_enter();
}
