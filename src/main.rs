mod helloworld;
mod elf;
mod comms;
mod fauna;
mod surroundings;

use crate::helloworld::io;
use crate::elf::calories;
use crate::elf::camp;
use crate::elf::rucksacks;
use crate::elf::cleanup;
use crate::comms::msg;
use crate::fauna::trees;
use crate::surroundings::bridge;

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
    let n_processed_characters_packet = msg::get_n_processed_characters_packet("data/day6_data/data.txt").unwrap();
    println!("There are {} characters that needed to be processed for a package.", n_processed_characters_packet);

    let n_processed_characters_msg = msg::get_n_processed_characters_msg("data/day6_data/data.txt").unwrap();
    println!("There are {} characters that needed to be processed for a message.", n_processed_characters_msg);

    println!("=================== Day 7 ===================");

    println!("=================== Day 8 ===================");
    let n_visible = trees::get_n_visible("data/day8_data/data.txt").unwrap();
    println!("There are {} trees visible.", n_visible);

    let scenic_score = trees::get_scenic_score("data/day8_data/data.txt").unwrap();
    println!("There is a spot with a scenic score of {}.", scenic_score);

    println!("=================== Day 9 ===================");
    let n_visited = bridge::get_n_visited("data/day9_data/data.txt").unwrap();
    println!("There were {} points visited.", n_visited);

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
