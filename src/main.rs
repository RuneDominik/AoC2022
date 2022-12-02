mod helloworld;
mod elf;

use crate::helloworld::io;
use crate::elf::calories;
use crate::elf::camp;

fn main() {
    println!("=================== Day 0 ===================");
    let msg = io::get_msg("data/day0_data/test_data.txt").unwrap();
    println!("Let us start this AoC with a happy {}", msg);

    println!("=================== Day 1 ===================");
    let max_cal = calories::get_max_calories("data/day1_data/data.txt").unwrap();
    println!("One elf has {} calories.", max_cal);

    let top3_cal = calories::get_top3_calories("data/day1_data/data.txt").unwrap();
    println!("One top three elves have {} calories.", top3_cal);

    println!("=================== Day 2 ===================");
    let score = camp::get_rps_score("data/day2_data/data.txt").unwrap();
    println!("The total score is {}.", score);

    println!("=================== Day 3 ===================");

    println!("=================== Day 4 ===================");

    println!("=================== Day 5 ===================");

    println!("=================== Day 6 ===================");

    println!("=================== Day 7 ===================");

    println!("=================== Day 8 ===================");

    println!("=================== Day 9 ===================");

    println!("=================== Day 10 ===================");

    println!("=================== Day 11 ===================");

    println!("=================== Day 12 ===================");

    println!("=================== Day 13 ===================");

    println!("=================== Day 14 ===================");

    println!("=================== Day 15 ===================");

    println!("=================== Day 16 ===================");

    println!("=================== Day 17 ===================");

    println!("=================== Day 18 ===================");

    println!("=================== Day 19 ===================");

    println!("=================== Day 20 ===================");

    println!("=================== Day 21 ===================");

    println!("=================== Day 22 ===================");

    println!("=================== Day 23 ===================");

    println!("=================== Day 24 ===================");
}
