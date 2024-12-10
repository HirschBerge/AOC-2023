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

### Benchmarks

#### Pre-Optimization
```
Benchmark 1: target/debug/day_2
  Time (mean ± σ):     931.0 ms ±   4.4 ms    [User: 929.8 ms, System: 1.2 ms]
  Range (min … max):   925.9 ms … 938.9 ms    10 runs
 
Benchmark 2: target/release/day_2
  Time (mean ± σ):      69.2 ms ±   0.8 ms    [User: 68.4 ms, System: 0.9 ms]
  Range (min … max):    68.0 ms …  71.3 ms    42 runs
 
Summary
  target/release/day_2 ran
   13.46 ± 0.16 times faster than target/debug/day_2
```
#### Post Post-Optimization

!todo()
