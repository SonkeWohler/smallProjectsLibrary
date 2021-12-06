This one was... different.  Not because the exercise was complicated - it's
just counting bits in different ways - but because without reasonable grasp on
ownership the code is a mess.  Part one worked with just some immutable
borrowing, but in part two I need to slice the vector into parts and that means
there are a bunch of immutable references in various places simultaneously.
For some reason that was giving me trouble.  Actually, the thing that was
giving me the most trouble was the types involved in this borrowing and the way
they are declared in the parameters to a function, to be technical.

Either way, I took some time to reread the ownership rules in the [learn rust
book](https://doc.rust-lang.org/book/title-page.html) and the funny thing is
that I read most of this before but only now that I was writing my own code do
I feel like I actually understand it.  I think now I can get back to following
the advent again.
