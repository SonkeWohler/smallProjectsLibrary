# Day 6

I didn't time myself, but this was one of those that was done in no time.  I
didn't learn any new Rust, just boosted my confidence to write it quickly.

The trick is obviously to keep track of the number of fish that are a given age
rather than of each fish individually (like the instructions suggested you do).
For a while I wasn't looking forward to having to shift those numbers around
manually until I realised I could use a circular queue.  And that was it...

OK, for the second part I slapped `u64` in the code instead of the `u32` I
would use by default and it was enough.

Because Rust is so strong on safety the only runtime error I could possibly
have run into was index out of bounds, everything else was handled by the
compiler.  I quite prefer this way of doing it over the writability that
Python, Javascript and the like promise by lifting these safeties.  But I feel
even in Java I am not that good at ensuring I catch all errors at compile-time
rather than runtime.
