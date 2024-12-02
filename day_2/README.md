# Day 1!
## This one was kinda tough

### Part 1 was pretty easy, but part two, I just couldn't.
I was using keys in a HashMap to regex the key to the variable. I had it working until I realized that this problem didn't make sense in context of the "story." Mixed alphabetical numbers and you had to accept BOTH of them!? Preposterous!

Well, that stumped me. When I added hacky shit like `twone` and `sevenine` the real numbers stopped registering!. So since I have nothing to prove, I figured it would be better to see someone's code that actually works and learn from that.

I ended up ~~stealing~~ recreating [WilkoTom's](https://github.com/wilkotom/Aoc2023/blob/main/day01/src/main.rs) method for part 2. Very cool solution. Similar to what I was going for, just... Well with experience.

### Benchmarks!!
```
Benchmark 1: target/debug/day_1
  Time (mean ± σ):     280.5 ms ±   1.8 ms    [User: 279.2 ms, System: 1.3 ms]
  Range (min … max):   278.0 ms … 283.1 ms    10 runs
 
Benchmark 2: target/release/day_1
  Time (mean ± σ):     950.1 µs ±  95.2 µs    [User: 438.0 µs, System: 680.1 µs]
  Range (min … max):   717.1 µs … 1248.0 µs    818 runs
 
  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.
 
Summary
  target/release/day_1 ran
  295.22 ± 29.65 times faster than target/debug/day_1
```
