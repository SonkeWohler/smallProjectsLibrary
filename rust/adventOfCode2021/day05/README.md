# Day 5

I am starting to get comfortable with Rust on this small exercise scale.

Today I started from the input processing instead of the data, which I added as
needed.  Which allowed me to enjoy the very simple way that structs can be
defined almost on the fly and used without much boilerplate.  It is a very
convenient way to add structure to your code as you move a long.

As a result most of my structs are simple containers for data with
functionality loosely associated with them.  It almost structures itself at
this point, but I am curious how this scales for larger projects.  I feel like
modules will be quite useful to scale this sort of thing, splitting your larger
packages into projects almost as small as these exercises.  If they succeed at
that then Rust has a very convenient way to structure your code as you write
it, rather than having to plan it ahead of time and putting encapsulation in
your way before you even know what way you're going to go.

I guess this is not unique to Rust; Kotlin aims to add these conveniences to
the JVM, I have seen Scala trying to do the same with a more scientific
programming spin (compared to Kotlin), Go has a bunch of convenience concepts
and aims to be more analogous to Java than Rust does.  This is just how
languages post 2000 (or perhaps 2010?) have been designed but languages from
before then may struggle to adopt these features in a way that truly adds
convenience rather than complexity and inconsistency.  I hear Python struggles
the least of the pre-2000 languages , but I have my reservations about Python
for large codebases.

Anyway, today I played around with Traits and Enums, in a very convenient way.
They simplified my code a lot and I think with a bit of work it would basically
follow the idea that "*good code describes itself, comments are unnecessary*".
I don't follow this philosophy myself, but in Rust I do find myself commenting
less than I feel I would in Java, because my intentions are much more clearly
captured by the syntax of the language.  In Java I would get lost in
boilerplate and inconveniences that would necessitate some kind of description
(usually javadoc at least) to cut down on the time it takes to understand what
is going on.

The exercise itself was kept simplistic to make it more doable.  First there
were no diagonals and when they came up they were only 45 degrees, so there was
very little to actually do but count through them.  A hashmap was very
convenient to keep time-complexity and code complexity down, which also meant
that adding diagonals was straight forward.  The only really messy code I have
is reading the input.

When writing this I don't even really remember the issues I had anymore.  One
issue had to do with lifetimes, which I have yet to get to.  The logic why they
exist follows straight forwardly from the ownership-system, but I'll stay off
them until I have to.  They would allow me to make my code more clean
especially because I could split it into smaller functions then, but until that
point I don't think it's that bad.

I guess the other issue was with trying to keep the conditions for vent-line
directionality simple.  I hate the kind of code that has an endless nesting of
`if` and some switches thrown in to make it inconsistent.  They get hard to
wrap your head around.  Their only positive is that you can through them
together relatively quickly and usually your don't have to tough them ever
again.  You enclose them in a function add a descriptive doc statement and know
that nobody will ever make changes to it.  You can make sure it works with unit
tests and call it a day.

This is where it was convenient to first ensure vent-lines always go down or
left to right if horizontal and to use enums to remember that.

