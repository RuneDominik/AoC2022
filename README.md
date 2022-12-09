# AoC2022
My Solutions to the [Advent of Code 2022](https://adventofcode.com/)

~~This year I'm using [The Rune Programming Language (ᚣ)](https://github.com/google/rune) for obvious reasons.~~

This year I'm using Rust, since Rune does not seem to support file i/o yet. Much sad :(

## Some remarks on my solutions:
(Just what I thought I should do better but didn't bothered to implement after having a running script)

### [Day 1](https://github.com/RuneDominik/AoC2022/blob/main/src/elf/calories.rs):
I solved this by chaining iterators and using build-in functions to manipulate the input as needed. Since I'm iterating over the whole input or manipulates versions of it multiple times, this could speed up by combining the actions and thus minimizing the amount of needed iterations. For part 2 I even used a sorting algorithm to find the three elves with the most calories. Speeding this up should be simple as one iteration would suffice when you simply remember the three highest entries.

### [Day 2](https://github.com/RuneDominik/AoC2022/blob/main/src/elf/camp.rs):
This could have been solved in many fascinating ways. I choose not to do so and simply parsed and mapped the input to the corresponding output. Works fine but would hardly be maintainable if one chooses to include more moves, maybe for a quick tournament of [Rock-Paper-Scissors-25](https://umop.com/rps25.htm)?

### [Day 3](https://github.com/RuneDominik/AoC2022/blob/main/src/elf/rucksacks.rs):
Maybe regroup the input stream in part 2 directly when reading it would be nicer so this could just be parsed completely to `map_three_rucksacks`, otherwise this looks good to me performance-wise.

### [Day 4](https://github.com/RuneDominik/AoC2022/blob/main/src/elf/cleanup.rs):
This has been straight-forward. Some dtypes are needlessly big, but I didn't felt like converting them in the end.

### [Day 5](https://github.com/RuneDominik/AoC2022/blob/main/src/elf/stacking.rs):
Did not solved it yet, parsing the input is a bit tricky.

### [Day 6](https://github.com/RuneDominik/AoC2022/blob/main/src/comms/msg.rs):
Quite hard-coded, quite the code duplication. Just introducing a variable indicating how long the sub-string has to be without duplicates would eliminate 2 of the functions. Also, I could easily break the counting for-loops when checking uniqueness as soon as I find a value that is not unique.

### [Day 7](https://github.com/RuneDominik/AoC2022/blob/main/src/comms/dirs.rs):
Let us not talk about it, ok? I really tried to implement a tree with pointers and everything, but Rust is a tad annoying if more then one mutable pointer points towards the same object. I guess my solution is quite close to right (at least for the first part of the day), but I'm missing something. Maybe hide everything in `RefCells` or write functions that pass every `register_xyz` down through the whole directory structure? One way or another, not so pretty. Anyway, feel free to take a look at [my solution](https://github.com/RuneDominik/AoC2022/blob/main/src/comms/dirs.rs), although it does not (yet) work.

### [Day 8](https://github.com/RuneDominik/AoC2022/blob/main/src/fauna/trees.rs):
Solved both parts by recursion. Bit of code-doubling (one function for each direction) but works. Although because of recursion and testing each and every tree this might not scale too good.

### [Day 9](https://github.com/RuneDominik/AoC2022/blob/main/src/surroundings/bridge.rs):
I felt like doing some object-oriented stuff today, so I did. This is of course not needed here and storing every visited point as a whole object will cause overhead and all, bit at least I was able to implement it. Turns out my part one solution cannot be extend for part two that easily. Since I couldn't allocate the necessary time on my to-do heap needed to rewrite this, I skipped part two for now.

## Please note:
This README contains links to external resources, for whose content I'm neither responsible nor liable. Click at your own risk.
