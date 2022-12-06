#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
    path::Path,
};


fn is_unique_packet(sl: &str) -> Result<bool, Error> {
    let mut counts = 0;
    
    for c in sl.chars() {
        counts += sl.matches(c).count();
    }

    if counts == 4 {
        return Ok(true);
    }else {
        return Ok(false);
    }
}

fn is_unique_msg(sl: &str) -> Result<bool, Error> {
    let mut counts = 0;
    
    for c in sl.chars() {
        counts += sl.matches(c).count();
    }

    if counts == 14 {
        return Ok(true);
    }else {
        return Ok(false);
    }
}


fn process_packet(inp: &str) -> Result<u64, Error> {
    let unrec_error = Error::new(ErrorKind::Other, "No Marker Detected!");

    for (i, _) in inp.chars().enumerate() {
        if i > 2 {
            let lookup = &inp[i-3..i+1]; //slice the last three chars

            if is_unique_packet(lookup).unwrap() {
                return Ok((i as u64) + 1)
            }
        }
    }

    return Err(unrec_error);
}

fn process_msg(inp: &str) -> Result<u64, Error> {
    let unrec_error = Error::new(ErrorKind::Other, "No Marker Detected!");

    for (i, _) in inp.chars().enumerate() {
        if i > 13 {
            let lookup = &inp[i-13..i+1]; //slice the last three chars

            if is_unique_msg(lookup).unwrap() {
                return Ok((i as u64) + 1)
            }
        }
    }

    return Err(unrec_error);
}


pub fn get_n_processed_characters_packet(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let inp = br.lines().next().unwrap().unwrap();

    let ind = process_packet(&inp).unwrap();

    Ok(ind)
}

pub fn get_n_processed_characters_msg(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let inp = br.lines().next().unwrap().unwrap();

    let ind = process_msg(&inp).unwrap();

    Ok(ind)
}