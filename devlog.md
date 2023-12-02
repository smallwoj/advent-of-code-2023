# Devlog
Documenting my journey for each day.

## [Day 1](src/bin/01.rs)
Nice short and sweet day.

### Part 1: 00:21:01 Rank 8421
Nothing too crazy here, some reversing and whatnot. Probably not the most efficient, could prolly do it all in one loop, but alas.

### Part 2: 01:32:14 Rank 9485
This one took a bit. I thought I could go ahead with just replacing them one at a time, starting at `one` and going up to `nine`, but that was proving problematic, 
as something like `eightwo` would turn into `eigh2` even though the eight came first. As such, I came up with the slicing method, which would replace each
word with the number.

All was good, until the `eightwo` -> `82` attacked. I definitely wouldn't have figured this out without the help of Kind Stranger,
so many thanks Zefick for the [hint](https://www.reddit.com/r/adventofcode/comments/1884fpl/2023_day_1for_those_who_stuck_on_part_2/).

After that, things worked out fine.

## [Day 2](src/bin/02.rs)
A pretty fun day, with a bit of tedium.

### Part 1: 01:02:52 Rank 11752
Took my time with this one. This day introduces one of the most important parts of Advent of Code, *String Processing*! Taking that input and turning it
into something not just workable but nice to work with as well is something I always have fun with, even if it slows me down a ton. 

Shout-out to the [regex crate](https://docs.rs/regex/latest/regex/) which, as it stands, is currently the only external crate installed,
though that may change later down the line if stuff like multiprocessing becomes relevant. I'm a big fan of the capture groups and you'll likely be seeing a ton
more of that as the days go by.

Plenty of splits going off in the subsets, and I always love an opportunity to use the `match` statement, very nice to use.

After all that work organizing the input, the actual solution to the puzzle is pretty trivial.

### Part 2: 01:14:37 Rank 11530
The work spent organizing the input paid off, as I can just jump right into the new logic.

Not a whole ton to say, the minimum cubes needed for each colour is the highest number pulled.
