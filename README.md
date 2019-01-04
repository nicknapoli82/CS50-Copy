# CS50-Copy
A rust version of CS50s copy program

Really just that. A simple, almost line for line, implimtation of CS50 copy.c program in Rust.
Thats it.

I left the original c code in the file hopefully for ease of reference.
I also pretty much intentionally made the Rust code very verbose. 

This code is not intended to be fast, or optimized, but a note.
Compile with rustc optimized, EG:
rustc -O copy.rs

The reason why is there is one line of code which will produce an error based on possible integer overflow.
There is a function that I could call that would allow for this error to not happen, I just didn't do it this way.
