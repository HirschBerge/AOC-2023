# Day 6
## I don't think I've done this for a bit. These last few were kind of just me learning how to parse. I still SUCK.

## Today, I fumbled my way through parsing. 

Then just generally followed Chris's parsing formula. Everything else was me. He does it way better and way faster, but I wanted to try loops, lmao

### Part 1. Super simple. If it's higher, `+= 1;` !!

### Part 2. Change the Vecs to just be usizes and go over them. lol... 

I didn't realize that dbg!ing in this would use ***that*** much RAM. I didn't pay attention and used all 64 GB and had to reboot. Woops!

I did a cool little trick where i started from the beginning, and once I hit a winning number, 
I added the remaining numbers to the total, then looped back from the end and subtracted 1 until I hit a winning value again and then broke out of all loops.

#### Benchmarks!!!

This is comparing going over the entire loop vs my trick to cut down on the iterations. Percentage wise, it's pretty impressive reduction.

```
Benchmark 1: Not Smart Method
  Time (mean ± σ):      29.8 ms ±   0.4 ms    [User: 29.2 ms, System: 0.7 ms]
  Range (min … max):    28.3 ms …  32.8 ms    1000 runs
 
Benchmark 2: Smart Method
  Time (mean ± σ):       7.9 ms ±   0.1 ms    [User: 7.3 ms, System: 0.7 ms]
  Range (min … max):     7.0 ms …   9.5 ms    1000 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Summary
  Smart Method ran
    3.77 ± 0.09 times faster than Not Smart Method
```
