# Devlog
Documenting my journey for each day.

## Day 1
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
