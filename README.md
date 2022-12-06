# AoC2022
My Solutions to the [Advent of Code 2022](https://adventofcode.com/)

~~This year I'm using [The Rune Programming Language (ᚣ)](https://github.com/google/rune) for obvious reasons.~~

This year I'm using Rust, since Rune does not seem to support file i/o yet. Much sad :(

## Some remarks on my solutions:
(just what I thought I should do better but didn't bothered to implement after havin a running skript)

### Day 1:
I solved this by chaining iterators and using build-in functions to manipulate the input as needed. Since I'm iterating over the whole input or manipulates versions of it multiple times, this could speed up by combining the actions and thus minimizing the amount of needed iterations. For part 2 I even used a sorting algorithm to find the three elves with the most calories. Speeding this up should be simple as one iteration would suffice when you simply remember the three highest entries.

### Day 2:
This could have been solved in many fascinating ways. I choose not to do so and simply parsed and mapped the input to the corresponding output. Works fine but would hardly be maintainable if one chooses to include more moves, maybe for a quick tournament of [Rock-Paper-Scissors-25](https://umop.com/rps25.htm)?

### Day 3:
Maybe regroup the input stream in part 2 directly when reading it would be nicer so this could just be parsed compleatly to `map_three_rucksacks`, otherwise this looks good to me performance-wise.

### Day 4:
This has been straight-forward. Some dtypes are needlessly big, but I didn't felt like converting them in the end.

### Day 5:
Did not solved it yet, parsing the input is a bit tricky.

### Day 6:
Quite hard-coded, quite the code dublication. Just introducing a variable indicating how long the sub-string has to be without dublicates would eliminate 2 of the functions. Also, I could easily break the counting for-loops when checking uniqueness as soon as I find a value that is not unique.

## Please note:
This README contains links to external resources, for whose content I'm neither responsible nor liable. Click at your own risk.
