# eatmemory
This is a program that can allocate ("eat") an amount of memory so behavior of the system can be tested.

The memory is allocated as anonymous memory.

# usage
This program takes the following arguments:
```
eatmemory 0.2.0

USAGE:
    eatmemory [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -m, --memlock      if set try to call mlock()
    -q, --query        if set displays memory and swap details
    -V, --version      Prints version information
    -v, --verbosity    if set displays memory and swap details plus actions

OPTIONS:
    -a, --alloc-definition <alloc-definition>
            size of memory (paged in) in MB, megabytes. -1 means equal to size_definition [default: -1]

    -s, --size-definition <size-definition>      size of allocation in MB, megabytes [default: 0]
```

Use `-q` to get an overview of physical and swap allocations.  
Use `-s` with a size in MB to allocate memory. This is the virtual memory allocation.  

By default, `-a` is set to -1, which means it will follow the size of `-s` and touch the pages to get these paged in.
It can be set to another value to show or investigate the difference between the virtual allocation and truly paged memory.  

Another option is to use the `-m` switch. This will execute (libc) mlock(). 
This might not succeed if the linux user has a lower limit set for locked memory than is requested. eatmemory will tell you.
Locked memory cannot be swapped.

Any comments, remarks or advise is welcome.

# acknowledgement
This is the rust version of the the eatmemory.c program by Julio Viera (https://github.com/julman99/eatmemory.git).
