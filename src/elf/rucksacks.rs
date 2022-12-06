#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
    path::Path,
};

fn map_item(c: char) -> Result<u64, Error> {
    let unrec_char_error = Error::new(ErrorKind::Other, "Unrecognized char!");

    if c.is_lowercase() {
        return Ok(c as u64 - ('a' as u64) + 1);
    } else if c.is_uppercase() {
        return Ok(c as u64 - ('A' as u64) + 27);
    } else {
        return Err(unrec_char_error);
    }
}

fn map_rucksack(inp: &str) -> Result<u64, Error> {
    let unrec_error = Error::new(ErrorKind::Other, "No doubled items!");

    let n_items = inp.chars().count() / 2;

    let (first, last) = inp.split_at(n_items);

    for c in first.chars() {
        if last.find(c).is_some() {
            return map_item(c) // minus 10 as to_digit converts 0..9 to int(0..9) first
        }
    }

    Err(unrec_error)
}

pub fn get_priorities(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let priorities: Vec<u64> = br.lines().map(|x| map_rucksack(&x.unwrap()).unwrap()).collect::<Vec<_>>();

    let rps_score: u64 = priorities.iter().sum();

    Ok(rps_score)
}