mod helloworld;
mod elf;

use crate::helloworld::io;
use crate::elf::calories;
use crate::elf::camp;
use crate::elf::rucksacks;
use crate::elf::cleanup;

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

    let score_corr = camp::get_rps_score_corrected("data/day2_data/data.txt").unwrap();
    println!("With the correct strategy guide, the total score is {}.", score_corr);

    println!("=================== Day 3 ===================");
    let priorities = rucksacks::get_priorities("data/day3_data/data.txt").unwrap();
    println!("The priority sum is {}.", priorities);

    let sticker_priorities = rucksacks::get_sticker_priorities("data/day3_data/data.txt").unwrap();
    println!("The sticker priority sum is {}.", sticker_priorities);

    println!("=================== Day 4 ===================");
    let n_matching_pairs = cleanup::get_n_matching_pairs("data/day4_data/data.txt").unwrap();
    println!("There are {} pairs that need immediate reassignment.", n_matching_pairs);

    let n_overlapping_pairs = cleanup::get_n_overlapping_pairs("data/day4_data/data.txt").unwrap();
    println!("There are {} pairs that need immediate reassignment.", n_overlapping_pairs);

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
