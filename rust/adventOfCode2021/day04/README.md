# Day 4

This exercise was particularly suited to using structs in my opinion.  Which
was incidentally the next chapter in the learn rust book.

Initially I wanted to use matrices for the draws but I found that it was
probably easier to check whether a row or column was full with integers, each
bit representing an entry.  Even duplicating that data isn't too much memory
use compared to a matrix of bools, I believe, and worth the simplicity, and if
speed was any consideration it would be worth that too.  I think usually speed
improvements, even very small ones, can have an impact on the total number of
processes that can be handled on a server per unit of time, while memory simply
needs to be kept within the available limits.

This time I got quite distracted a lot.  I have a life, what can I say.

I do feel that the way I was doing things in a more OOP inspired way was
holding me back a little at times.  I would like to look at other people's
solutions to gage how realistic that feeling is.  Perhaps it has to do with the
criticisms of OOP that it adds too much overhead for too little gain
(especially with something this small) or perhaps it was just the fact that I
kept being distracted and had difficulty preserving continuity of thought.

Debugging was certainly a bit annoying because whatever output (println or
break point in an IDE) was not verbose enough.  Rather than improving its
display I chose to use pen and paper to understand what was going wrong, which
was as usual not where I initially thought I would have made a mistake.  I
thought the way I represented the draws as binary numbers would be an issue,
but aside from a `+1` in `main` that worked great.  That is actually where I
was slaking the most, I missed the fact that the file doesn't end on a new line
(face-palm).  Somehow the smaller part of the code, the `main` had the most
errors.  I wonder why?

Well, maybe the OOP I am used to isn't so bad after all.  Writing this text
made me realise that.

## But what about OOP in Rust?

Rust is of course not a purist OOP language.  It has `structs` which have
values (primarily) and methods (also), and these can be instantiated, but that
is pretty much where the OOP ends.  `structs` are primarily a way to structure
data (hence the name) with methods tagged onto them after the fact (`impl`),
which is a nightmare for traditional OOP.

Well, to make it more like Java you could start by keeping each `struct` in its
own file and require methods to only be defined in that file, nowhere else.
You'd also have to require your programmers to not use `struct` values
directly, but to go through methods.

But Rust isn't supposed to be used that way.  Encapsulation is the first thing
to be knocked down a peg.  It is implemented on a module level rather than the
`struct`.  Modules are a natural way to split up your code into logically
separate parts in ways that `structs` or classes are simply too fine grained.
It seems to be expected that as you work on the code on a module level you have
the rest of the module in mind.  And as such you should be able to enjoy full
access to every part of your code this way.  No getters and setters, no
boilerplate to share a variable between many classes.  That latter part is not
necessary because we already worry about shared data on a memory allocation
level with the ownership system , so adding boilerplate to handle it on a class
level would be utterly redundant.

Being able to implement new methods on top of existing `structs` on the fly,
even (I believe) those that you imported from elsewhere, means your
polymorphism can take off like crazy.  Which is something I do worry about for
larger projects.  How do you keep track of what methods have been added to your
`struct` with `impl` and what hasn't.  Well, thinking about it its all about
the imports and the `impl`.  If they are organised well enough then you should
at least know exactly where to look for that information, plus a good debugger
will still tell you.

Oh, constructors don't exist, you just define **all** the values of the
instance and you may define functions to do that for you.  So factories are a
thing, but constructors aren't.  I am not sure if that is going to make larger
`structs` more difficult, considering that constructors are really just special
methods.

## Also learnt

Well, I think it was in this exercise that I realised what it means that Rust
is an expression based language.  Everything, almost everything can be a value
if you leave out the `;`.  Writing `false` instead of `return false;` is not so
groundbreaking, and being able to write `if <function> {...` with the
`function` doing something beyond just returning a value (like I used with
`Board.draw(value)` is not new to me.  But surprisingly these things do come in
handy when writing conditionals and `match` statements quite a bit.  It helps
clean up the code of helper variables and make it more straight forward what
the reasoning is, in my opinion.  I basically see a more straight forward path
when the data is being processed from where the data starts out to where it is
going.

This kind of reminds me of *Bash*, where everything is a function, really
everything, and a function always returns something.  At the very least bash
returns `0` for the function having been successful, and something similar
exists in Rust with `()`.

## Final Thoughs for the day

Well, I was more successful with my thought process than I thought when I
started running into bugs.  My construction of `structs` was actually working
as planned and my runtime errors had little to do with Rust's peculiarities.
That is a recurring pattern, errors because I don't understand the language
tend to be compile errors while runtime errors tend to be the small things like
mixing up a `==` and a `!=` or forgetting to add a `+ 1` when counting stuff.
Small logic mistakes that happen.  No null pointers or type conversions or any
such things.

And this is exactly what Rust was promising, finding errors at compile time
rather than runtime as much as possible.  So far I am still very pleased.
