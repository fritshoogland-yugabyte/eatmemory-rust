# eatmemory
This is a program that can allocate ("eat") an amount of memory so behavior of the system can be tested.

The memory is allocated as anonymous memory.

# usage
This program takes the following arguments:
```
eatmemory 0.4.0

USAGE:
    eatmemory [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -q, --query        if set displays memory and swap details, then quits
    -s, --step         if set stops for every action to allow investigation
    -V, --version      Prints version information
    -v, --verbosity    if set displays memory and swap details plus actions

OPTIONS:
    -a, --alloc-type <alloc-type>        type of allocation [default: native]  [possible values: native, native-wait,
                                         mlock, malloc, mmap]
    -i, --init-size-mb <init-size-mb>    size of initialisation/creation in MB, megabytes. This is the virtual set size
                                         [default: 0]
    -u, --use-size-mb <use-size-mb>      size of used/touched in MB, megabytes. This is the resident set size. By
                                         default, the init size is allocated
```

Use `-q` to get an overview of physical and swap allocations only.  

Use `-i` with a size in MB to allocate memory. It is required to set `-i` to a nonzero value to make eatmemory allocate memory. This is the virtual memory allocation.  

By default, the value of `-i` will set `-u` to the same value if `-u` is not set, making the entire allocation to be set to zero and thus be paged in.

Use `-u` with a size in MB to allocate an amount of memory different and lower than set with `-i`, which can be as low as 0 (to allocate but not touched and thus not pages in).
This can be used to show the difference between the virtual set size and the resident set size.

Use `-v` to make eatmemory more verbose, which shows what it is doing, as well as the pointer to its allocation. By default, it will just do what it's asked.

Use `-s` to let eatmemory pause waiting for enter after virtual allocation/creation, touching the memory/set it to zero, and removing the allocation. By default, eatmemory will only pause waiting for enter after creation and setting it to zero.

use `-a` to choose the allocation method:

- native: a Rust vec (vector) is allocated with a single element which is unsigned 8 bits, so the number of elements represents the size in bytes. The use is setting these elements to zero. This is the default.   
- native-wait: a Rust vec (vector) is allocated with a single element identical to the 'native' method, the difference is the part of setting the elements to zero with the use setting, which is done after 2 seconds. This allows multiple eatmemory processes to take advantage of linux overcommit and allocate more virtual memory than is available, after which the processes set the contents to zero, which leads to allocating this memory, potentially allocating more memory than is available.  
- mlock: a Rust vec (vector) is allocating with a single element which is unsigned 8 bits. The use setting calls the libc mlock() function for the size of the use setting.
- malloc: the libc malloc() function is called to allocate memory for the size of init, then libc memset() is used to set use size bytes to zero, and then libc free() is called to deallocate the memory.
- mmap: the libc mmap() function is called to allocate anonymous memory for the size of init, then libc memset() is used to set use size bytes to zero, and then libc munmap() is called to deallocate the memory.

Any comments, remarks or advise is welcome.

# acknowledgement
This utility is inspired by the eatmemory.c program by Julio Viera (https://github.com/julman99/eatmemory.git).

# installation
In order to run this utility, you must compile it.  
The compilation require's Rust's Cargo: https://www.rust-lang.org/tools/install (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)  
Git clone this repo, cd into it, and run `cargo build --release`. That compiles the code.  
After it's compiled, you can run it in the following way:`./target/release/eatmemory`.
