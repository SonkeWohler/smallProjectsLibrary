# Day 7

This one threw me off at first.  At first I wanted to take the average, but
that obviously wouldn't work with the example.  It turns out that it doesn't
matter how far each crab is away from the alignment point, it only matters that
you separate the population of crabs into halves on each side of that point.

This is easily demonstrated by adding a new crab to the example at position
`1000`.  It won't affect the alignment point, because moving the alignment
point further to the right would mean all crabs to the left of `2` would have
to each spend extra fuel for each one fuel that the one crab at `1000` would be
saving.  Continue this logic and you realise that you are looking for the
*median* of the set of crabs, not the average.

If we would care about minimising the fuel burden on any individual crab we
would probably care about the average more, but since we are concerned only
with the total cost we don't care if one crab has to move a lot.

This is the first advent of code day that I look things up on the algorithm I
want to use, rather than just on the Rust language.  And so it turns out there
are three possible ways of doing it today:

### The Most Naive approach

You can, of course, read in the crabs, arrange them on a number line with the
number of crabs in each position and then calculate the total fuel cost for
each position on your number line.  Say we shift the crabs so that the lowest
number is `0` and the largest is `n`, then this has time complexity `n^2`.  In
space we can potentially save complexity by storing only the number of crabs in
each position rather than the position of each crab, but this depends a little
on the distribution.

Lets also say that the number of crabs is `m`.

### The Naive Median Approach

Since I am concerned with time complexity, simply for the heck of it, the
slightly less naive approach is to sort the crabs into an array and then select
the median.  Selecting the median from an ordered set would take constant time,
but to convert the input into a sorted input takes us `m log(m)`, plus you need
all `m` entries in memory.

This is, however, the easiest approach to implement, since sorting comes with
the `std::collections` libraries.  If this was a larger project I would start
with this one and consider optimising it further if that is necessary.

### The Fastest Approach

If `m log(m)` time is not good enough there are
[approaches](https://stackoverflow.com/questions/10662013/finding-the-median-of-an-unsorted-array)
that take linear time and even ways to deal with memory complexity.  I won't
bother today, but this would be a nice little exercise as well.

## Part 2

As I already suggested, when the fuel consumption of each crab becomes
important the average becomes more important.  I thought this would be because
the crabs would have some maximum amount of fuel with them and you'd have to
find a value below that, but this also works.  Now the total fuel consumption
is proportional to the distances each crab covers, rather than simply the total
distance covered by all crabs.

This seems to work very well in the small scale examples provided and that I
tested it on, but I seem to be off by one position in the real test.  Unless I
miscalculate the average by `<1` something about the rounding doesn't seem to
add up.  Removing parts of the input mostly gets me to the same situation where
the average dictates some number `x.51...` which gets rounded up, while the
exhaustive method shows I should be rounding down.  I am not quite sure how I
should be able to tell I should be rounding down here, and I am not too
concerned.  At worst I could be using the average to estimate the position and
then search the area around it for the true answer.

In this way the time complexity for the second part (with corrected fuel costs)
is pretty much `2m`, since we don't need to sort as for the median and we can
calculate both the average and the fuel consumption in `m` time.  We have to do
them consecutively, but beyond that even adding a scan for the `10` positions
around the average won't meaningfully impact performance.

Figuring out why the average and the true optimal position are exactly one
apart would be more interesting to figure out on paper, but not now.  I am
practicing Rust not algos or provability right now.

## Rust

Today I didn't really do anything new in Rust.  I fiddled a little with type
casting and dereferencing, but both follow quite naturally from the type and
borrowing systems.  What I am more interested in are the ways I can pre-empt
type casting errors, but today I wasn't quite in the mood for that yet.

I am still quite happy with the way that I have pretty much no runtime errors.
It is quite satisfying and I don't find that I am really struggling with the
compiler.  Rather I feel the compiler is helping me not to struggle with
obscure runtime errors that always annoyed me in the best cases.

I have yet to apply lifetimes though, which I guess would be holding me back in
a larger project.  Perhaps I should focus on finding ways to practice that
instead.
