# Day 2!!
## Phew! After yesterday's assignment, this was really a fun breath of fresh air.
I'm sure I can (and probably will) optimize this a lot more
### Part 1
Just looping through, grabbing the values for everything out w/ regex, and manipulating that info was really neat.
I did have some trouble with the syntax for checking to see if *any* of the values in the Vec were higher than the max cubes, so I did result in have CGPT help me with that, but it's a learning tool. 
I just can't ***rely*** on it.

Though, putting a break statement really cost me some time in P2...
### Part 2

My method of pulling each color's values into a vec from P1 saved me a good bit of time with P2. Inspite of the fact I had an break. I was able to just get the max of each color by doing `let max = color_count.iter().max().unwrap();` in the individual color's loop.
This is wher having the [break](https://github.com/HirschBerge/AOC-2023/blob/main/day_2/src/main.rs) when setting `invalid_game` cut off a solid 30 mintues of `dbg!()`ing. I forgot that I had a break there, so the final `max` was not being multipled into `cubed`.
Once I caught that, my tests passed and i was able to submit.
