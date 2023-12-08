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

To get around this, I simply replaced everything except the last character in the slice with the number, as the leftover character would both be a part of the
next number if the next set of characters is a number, or ignored if it isn't a valid number.

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

## [Day 3](src/bin/03.rs)
Wasn't able to start right at release time and had to step out during part 2, so the times are abysmal for this one. I tracked the time still though.

### Part 1: 17:47:45 Rank 71424
Actual time spent was 1:28 straight.

Some fun spent keeping track of the numbers and generating the valid ranges. I ran into some issues creating the ranges and ran into an underflow error when
trying to generate the ranges.

### Part 2: 22:44:58 Rank 66513
Actual time spent was 0:18 + another 0:10 for a total time of 1:56.

Some hash map fun here to keep track of the numbers the gears connect to. The only issue I ran into here was skipping over the character directly to the right of
the number sequence when creating the hash map of gears.

## [Day 4](src/bin/04.rs)

### Part 1: 00:43:24 Rank 12381
Some more text parsing. All the spaces between the numbers really messed with my regexes. I made a fools errand of trying to do some wacky
bitshifting, until I realized that we are doubling, not using powers of 2. Looking back I maybe could've done it, but alas.

### Part 2: 03:26:50 Rank 21131
This one gave me a ton of trouble. My regex around the game number was messing me up, and rust was giving me a metric ton of borrowing errors that lead me down a
rabbithole of changes needed so that the compiler doesn't yell at me regarding borrowing, only for me to revert the changes back and it working just fine.

I tried having the copy count in the card struct itself, but that wasn't working, so I settled on the hash map.

I also mistakenly started the problem directly on part 1, which resulted in it's share of problems.

In the end, the copy count was sufficient to have an efficient solution.

## [Day 5](src/bin/05.rs)
I think this is the first one I actually had a pretty big struggle to get through.

### Part 1: 02:11:04 Rank 13981
What a doozy. A ton of repeating code, many things to keep track of, and a tangible problem.

I fell for the source/destination range swap. I also was transforming the ranges to arrays, but that was giving me some issues, so I went ahead and made it use the 
range calculations.

### Part 2: 03:08:27 Rank 6850
Even more of a doozy. My hopes of just running the seeds individually were dashed pretty quickly by my lack of RAM. I did the reverse location bruteforce, which runs
about 7 seconds with release optimization, which I'll take.

## [Day 6](src/bin/06.rs)
Not a whole lot to say, just gotta figure out the quadratic formula for the optimal solution.

### Part 1: 00:42:38 Rank 10922
Took some time to figure out the formula needed, as well as parsing the input.

### Part 2: 00:56:11 Rank 11019
Toughest part here was combining the numbers together, but there's prolly a better way to do it.

## [Day 7](src/bin/07.rs)
This was a fun one, dealing with the comparators and all that. This time I put the structs in the solve function itself so that changes are easily made if
needed, which was needed today.

Was too tired to do it at the usual time, took a couple hours for the first part, and another 40 minutes for the second part.

### Part 1: 16:55:38 Rank 49845
Implementing the `cmp` function will likely be helpful in future days, so that's nice to have. I like the match solution for counting the distinct symbols as
well. Got mixed up with the sorting order that rust does, and it's kind of a shame you have to reverse it instead of just saying 'I want it sorted the other way'.

### Part 2: 17:38:02 Rank 44427
Definitely not the most efficient solution, but it was fun to do at least. Missed the part where the `J` became the lowest ranked card, which cost me some time.

