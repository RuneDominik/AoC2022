mod helloworld;

#[cfg(test)]
mod helloworld_tests {
    #[test]
    fn test_io() {
        use crate::helloworld::io;

        let iostream = io::get_msg("data/day0_data/test_data.txt").unwrap();

        assert_eq!(iostream, "Hello World!");
    }
}
