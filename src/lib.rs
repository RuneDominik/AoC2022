mod helloworld;
mod elf;

#[cfg(test)]
mod helloworld_tests {
    #[test]
    fn test_io() {
        use crate::helloworld::io;

        let iostream = io::get_msg("data/day0_data/test_data.txt").unwrap();

        assert_eq!(iostream, "Hello World!");
    }
}

#[cfg(test)]
mod elf_tests {
    #[test]
    fn test_calories() {
        use crate::elf::calories;

        let max_calories = calories::get_max_calories("data/day1_data/test_data.txt").unwrap();

        assert_eq!(max_calories, 24000);

        let top3_calories = calories::get_top3_calories("data/day1_data/test_data.txt").unwrap();

        assert_eq!(top3_calories, 45000);
    }
    #[test]
    fn test_camp() {
        use crate::elf::camp;

        let score = camp::get_rps_score("data/day2_data/test_data.txt").unwrap();

        assert_eq!(score, 15);

        let corr_score = camp::get_rps_score_corrected("data/day2_data/test_data.txt").unwrap();

        assert_eq!(corr_score, 12);
    }
    #[test]
    fn test_rucksacks() {
        use crate::elf::rucksacks;

        let priorities = rucksacks::get_priorities("data/day3_data/test_data.txt").unwrap();

        assert_eq!(priorities, 157);

        let sticker_priorities = rucksacks::get_sticker_priorities("data/day3_data/test_data.txt").unwrap();

        assert_eq!(sticker_priorities, 70);
    }
    #[test]
    fn test_cleanup() {
        use crate::elf::cleanup;

        let matching_pairs = cleanup::get_n_matching_pairs("data/day4_data/test_data.txt").unwrap();

        assert_eq!(matching_pairs, 2);

        let overlapping_pairs = cleanup::get_n_overlapping_pairs("data/day4_data/test_data.txt").unwrap();

        assert_eq!(overlapping_pairs, 4);
    }
    #[test]
    fn test_comms() {
        use crate::elf::comms;

        let n_processed_characters = comms::get_n_processed_characters_packet("data/day6_data/test_data.txt").unwrap();

        assert_eq!(n_processed_characters, 11);

        let n_processed_characters = comms::get_n_processed_characters_msg("data/day6_data/test_data.txt").unwrap();

        assert_eq!(n_processed_characters, 26);
    }
}