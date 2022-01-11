# eatmemory
This is a program that can allocate ("eat") an amount of memory so behavior of the system can be tested.

The memory allocated is anonymous memory.

# usage
The program takes one argument, which is the amount of memory to allocate as megabytes:
```
$ target/debug/eatmemory 10
total memory: 828 MB, available memory: 532 MB, amount to be allocated: 10 MB
Press enter to allocate.

Press enter to stop and deallocate
```
# acknowledgement
This is the rust version of the the eatmemory.c program by Julio Viera (https://github.com/julman99/eatmemory.git).
