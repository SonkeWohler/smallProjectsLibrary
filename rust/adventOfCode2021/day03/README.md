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

It was quite fun to go back to the bit moving I used to do in high-school,
actually.  But from a Rust perspective I am glad I am doing this advent of
code.  This time I got a pretty good hang of the ownership system and I am not
messing it up as much.  But the brilliant thing about it is that even if I had
messed it up the compiler would tell me **without having to do runtime
tests!**.

And this is what is so satisfying about Rust.  Runtime errors are
limited to truly unpredictable issues like users putting in letters instead of
numbers.  And on top of that you can guard against these situations with a
pretty satisfying system of `Option`, `Result` and similar structs.  It reminds
me of Java with its family tree of Exceptions that you can use to funnel your
errors into `catch-try` constructions that will save you from these issues.
Somehow I feel more satisfied by the way Rust does it here, but that can also
be hype in the moment.

It does have very satisfying ways to handle decisions (`match`, using if in
assignments - actually called expressions) which work well with the error
handling because errors are values like anything else.  You can reason over
errors much easier than you can in Java because they are part of the value.
Even the runtime errors are predictable in that they can only happen when you
`unwrap` something, otherwise your code is perfectly safe.

This doesn't mean that runtime errors are a thing of the past, but I am very
excited about the possibilities to avoid them with much more ease than I am
used to from Java.

All in all, once you wrap your head around these core concepts the whole
language feels very intuitive.  Like it is well thought out and kept
consistent.  You can't hold older languages to the same standard though,
because they have gone through a history of adding functionality on top of
features until you start to get unpredicted friction.  The same could happen to
Rust in another decade or two.
