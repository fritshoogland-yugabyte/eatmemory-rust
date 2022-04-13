# eatmemory
This is a program that can allocate ("eat") an amount of memory so behavior of the system can be tested.

The memory is allocated as anonymous memory.

# usage
This program takes the following arguments:
```
eatmemory 0.3.0

USAGE:
    eatmemory [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -m, --memlock      if set try to call mlock()
    -q, --query        if set displays memory and swap details, then quits
    -V, --version      Prints version information
    -v, --verbosity    if set displays memory and swap details plus actions

OPTIONS:
    -a, --alloc-definition <alloc-definition>
            size of memory (paged in) in MB, megabytes. By default, the entire size is allocated

    -s, --size-definition <size-definition>      size of allocation in MB, megabytes [default: 0]
```

Use `-q` to get an overview of physical and swap allocations.  

Use `-s` with a size in MB to allocate memory. It is required to set `-s` to a nonzero value to make eatmemory allocate memory. This is the virtual memory allocation.  

By default, the value set with `-s` will <u>allocate</u> and <u>write</u> the allocated memory, paging it in, and adding it to the process' resident set size (RSS).

Use `-a` with a size in MB to allocate an amount of memory different than set with `-s`. This can be used to demonstrate the difference between the virtual set size (VSZ) and resident set size (RSS).

Use `-v` to make eatmemory more verbose, and expose memory sizes and tell what it is doing, as well as the pointer to its allocation. By default, it will just do what it's asked.

use `-m` to use (libc) mlock(). 

Any comments, remarks or advise is welcome.

# acknowledgement
This utility is inspired by the eatmemory.c program by Julio Viera (https://github.com/julman99/eatmemory.git).

# installation
In order to run this utility, you must compile it. The compilation require's Rust's Cargo: https://www.rust-lang.org/tools/install (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
Git clone this repo, cd into it, and run `cargo build --release`. That compiles the code.  
After it's compiled, you can run it in the following way:`./target/release/eatmemory`.
