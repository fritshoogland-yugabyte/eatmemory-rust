extern crate libc;
use std::process;
use sysinfo::SystemExt;
use libc::{c_void, mlock, malloc, memset, free, mmap, munmap, PROT_READ, PROT_WRITE, MAP_ANONYMOUS, MAP_FAILED, MAP_PRIVATE};
use structopt::StructOpt;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long)]
    /// if set displays memory and swap details, then quits.
    query: bool,
    #[structopt(short, long)]
    /// if set displays memory and swap details plus actions.
    verbosity: bool,
    #[structopt(short, long)]
    /// if set stops for every action to allow investigation
    step: bool,
    #[structopt(short, long, default_value = "0")]
    /// size of initialisation/creation in MB, megabytes. This is the virtual set size.
    init_size_mb: i64,
    #[structopt(short, long)]
    /// size of used/touched in MB, megabytes. This is the resident set size. By default, the init size is allocated.
    use_size_mb: Option<i64>,
    #[structopt(short, long, default_value = "native", possible_values(&["native", "native-wait", "mlock", "malloc", "mmap"]))]
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
    // if use_size_mb (-u) is not set, it will be set to the size of init_size_mb (-i)
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

    let init_size_as_u8: usize = (options.init_size_mb*1024*1024).try_into().unwrap();
    let use_size_as_u8: usize = (use_size_mb*1024*1024).try_into().unwrap();
    println!("Initialization size: {}, Used/page in size: {}", &init_size_as_u8, &use_size_as_u8);

    match options.alloc_type.as_str() {

        "native" => {
            println!("native mode (creates vector)");
            // this is a separate scope for the vector, so the last wait can validate the memory being freed
            {
                // this is the virtual allocation
                let mut hog = vec![0u8; init_size_as_u8];
                let ptr = hog.as_ptr() as *const c_void;
                if options.verbosity { println!("1. vector created with size {}. pointer to vector: {:?}", &init_size_as_u8, ptr) }
                if options.verbosity { print_memory() };
                if options.step { wait_for_enter() };
                // this is paging in the vector
                hog[0..use_size_as_u8].fill(0);
                if options.verbosity { println!("2. vector set to 0 for {} bytes", &use_size_as_u8) }
                if options.verbosity { print_memory() };
                wait_for_enter();
            }
            if options.verbosity { println!("3. allocation out of scope") };
            if options.verbosity { print_memory() };
            if options.step { wait_for_enter() };

        },

        "native-wait" => {
            println!("native mode (creates vector) - allocation after 2 seconds");
            // this is a separate scope for the vector, so the last wait can validate the memory being freed
            {
                // this is the virtual allocation
                let mut hog = vec![0u8; init_size_as_u8];
                let ptr = hog.as_ptr() as *const c_void;
                if options.verbosity { println!("1. vector created with size {}. pointer to vector: {:?}", &init_size_as_u8, ptr) }
                if options.verbosity { print_memory() };
                if options.step { wait_for_enter() };
                // sleep
                sleep(Duration::from_secs(2));
                // this is paging in the vector
                hog[0..use_size_as_u8].fill(0);
                if options.verbosity { println!("2. vector set to 0 for {} bytes", &use_size_as_u8) }
                if options.verbosity { print_memory() };
                wait_for_enter();
            }
            if options.verbosity { println!("3. allocation out of scope") };
            if options.verbosity { print_memory() };
            if options.step { wait_for_enter() };

        },

        "mlock" => {
            println!("mlock mode (uses native mode/vector for the pointer to the allocation)");
            // this is a separate scope for the vector, so the last wait can validate the memory being freed
            {
                // this is the virtual allocation
                let hog = vec![0u8; init_size_as_u8];
                let ptr = hog.as_ptr() as *const c_void;
                if options.verbosity { println!("1. vector created with size {}. pointer to vector: {:?}", &init_size_as_u8, ptr) }
                if options.verbosity { print_memory() };
                if options.step { wait_for_enter() };
                // the mlock pages in the memory
                let result = unsafe { mlock(ptr, use_size_as_u8) };
                if options.verbosity { println!("2. mlock for {} bytes. result: {}", &use_size_as_u8, result) }
                if options.verbosity { print_memory() };
                wait_for_enter();
            }
            if options.verbosity { println!("3. allocation out of scope") };
            if options.verbosity { print_memory() };
            if options.step { wait_for_enter() };
        },

        "malloc" => {
            println!("malloc mode");
            // this is the virtual allocation
            let pointer = unsafe{ malloc(init_size_as_u8) };
            if pointer.is_null() {
                println!("malloc did not succeed, returned nullpointer.");
                process::exit(0);
            };
            if options.verbosity { println!("1. malloc for {} bytes. pointer returned by malloc: {:?}", &init_size_as_u8, pointer) }
            if options.verbosity { print_memory() };
            if options.step { wait_for_enter() };
            // memset pages in the memory
            let memset_pointer = unsafe { memset(pointer, 0, use_size_as_u8) };
            if memset_pointer.is_null() { println!("memset did not succeed, returned nullpointer.") }
            if options.verbosity { println!("2. memset for {} bytes. pointer returned by memset {:?}", &use_size_as_u8, memset_pointer) }
            if options.verbosity { print_memory() };
            wait_for_enter();
            // free releases the memory
            unsafe { free(pointer) };
            if options.verbosity { println!("3. calling free") };
            if options.verbosity { print_memory() };
            if options.step { wait_for_enter() };
        },

        "mmap" => {
            println!("mmap mode");
            // this is the virtual allocation
            let pointer = unsafe{ mmap(std::ptr::null_mut(), init_size_as_u8, PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE, 0, 0) };
            if pointer == MAP_FAILED {
                println!("mmap did not succeed, returned MAP_FAILED.");
                process::exit(0);
            };
            if options.verbosity { println!("1. mmap for {} bytes. pointer returned by mmap {:?}", &init_size_as_u8, pointer) }
            if options.verbosity { print_memory() };
            if options.step { wait_for_enter() };
            // memset pages in the memory
            let memset_pointer = unsafe { memset(pointer, 1, use_size_as_u8) };
            if memset_pointer.is_null() { println!("memset did not succeed, returned nullpointer.") }
            if options.verbosity { println!("2. memset for {} bytes. pointer returned by memset {:?}", &use_size_as_u8, memset_pointer) }
            if options.verbosity { print_memory() };
            wait_for_enter();
            // the munmap releases the memory
            let retval = unsafe { munmap(pointer, init_size_as_u8) };
            if retval < 0 { println!("munmap did not succeed, returned: {}", retval) }
            if options.verbosity { println!("3. calling munmap") };
            if options.step { wait_for_enter() };
        },

        &_ => unreachable!(),

    }
}
