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
}