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
            return map_item(c)
        }
    }

    Err(unrec_error)
}

fn map_three_rucksacks(r1: &str, r2: &str, r3: &str) -> Result<u64, Error> {
    let unrec_error = Error::new(ErrorKind::Other, "No doubled items!");

    for c in r1.chars() {
        if r2.find(c).is_some() && r3.find(c).is_some() {
            return map_item(c)
        }
    }

    Err(unrec_error)
}

pub fn get_priorities(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let priorities: Vec<u64> = br.lines().map(|x| map_rucksack(&x.unwrap()).unwrap()).collect::<Vec<_>>();

    let prio_score: u64 = priorities.iter().sum();

    Ok(prio_score)
}

pub fn get_sticker_priorities(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let mut rucksacks = br.lines();

    let mut shared_sticker = Vec::new();

    loop {
        let r1 = rucksacks.next();
        if r1.is_none(){
            break;
        }

        let r2 = rucksacks.next().unwrap().unwrap();
        let r3 = rucksacks.next().unwrap().unwrap();

        shared_sticker.push(map_three_rucksacks(&r1.unwrap().unwrap(), &r2, &r3).unwrap());
    }

    let prio_score: u64 = shared_sticker.iter().sum();

    Ok(prio_score)
}