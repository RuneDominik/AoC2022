#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};

fn match_sections(inp: &str) -> Result<u64, Error> {
    let v: Vec<u64> = inp.split(&['-', ','][..]).map(|x| x.parse::<u64>().unwrap()).collect();

    if ((v[0] <= v[2]) && (v[1] >= v[3])) || (v[2] <= v[0]) && (v[3] >= v[1]) {
        return Ok(1)
    } else {
        return Ok(0)
    }
}

fn overlap_sections(inp: &str) -> Result<u64, Error> {
    let v: Vec<u64> = inp.split(&['-', ','][..]).map(|x| x.parse::<u64>().unwrap()).collect();

    if v[1] < v[2] || v[0] > v[3] {
        return Ok(0)
    } else {
        return Ok(1)
    }
}

pub fn get_n_matching_pairs(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let mask: Vec<u64> = br.lines().map(|x| match_sections(&x.unwrap()).unwrap()).collect::<Vec<_>>();

    let pairs = mask.iter().sum();

    Ok(pairs)
}

pub fn get_n_overlapping_pairs(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    let mask: Vec<u64> = br.lines().map(|x| overlap_sections(&x.unwrap()).unwrap()).collect::<Vec<_>>();

    let pairs = mask.iter().sum();

    Ok(pairs)
}