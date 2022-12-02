#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
    path::Path,
};

fn map_game(inp: &str) -> Result<u64, Error> {
    let unrec_error = Error::new(ErrorKind::Other, "Unrecognized Combinations!");

    let mut split_inp = inp.split(' ');

    let game_tuple: (&str, &str) = (split_inp.next().unwrap(), split_inp.next().unwrap());
    
    match game_tuple {
        ("A", "Y") => return Ok(8),
        ("B", "Z") => return Ok(9),
        ("C", "X") => return Ok(7),
        ("A", "X") => return Ok(4),
        ("B", "Y") => return Ok(5),
        ("C", "Z") => return Ok(6),
        ("A", "Z") => return Ok(3),
        ("B", "X") => return Ok(1),
        ("C", "Y") => return Ok(2),
        _ => return Err(unrec_error),
    }
}

fn map_response(inp: &str) -> Result<u64, Error> {
    let unrec_error = Error::new(ErrorKind::Other, "Unrecognized Combinations!");

    let mut split_inp = inp.split(' ');

    let game_tuple: (&str, &str) = (split_inp.next().unwrap(), split_inp.next().unwrap());
    
    match game_tuple {
        ("A", "X") => return map_game("A Z"),
        ("A", "Y") => return map_game("A X"),
        ("A", "Z") => return map_game("A Y"),
        ("B", "X") => return map_game("B X"),
        ("B", "Y") => return map_game("B Y"),
        ("B", "Z") => return map_game("B Z"),
        ("C", "X") => return map_game("C Y"),
        ("C", "Y") => return map_game("C Z"),
        ("C", "Z") => return map_game("C X"),
        _ => return Err(unrec_error),
    }
}

pub fn get_rps_score(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let scores: Vec<u64> = br.lines().map(|x| map_game(&x.unwrap()).unwrap()).collect::<Vec<_>>();

    let rps_score: u64 = scores.iter().sum();

    Ok(rps_score)
}

pub fn get_rps_score_corrected(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let scores: Vec<u64> = br.lines().map(|x| map_response(&x.unwrap()).unwrap()).collect::<Vec<_>>();

    let rps_score: u64 = scores.iter().sum();

    Ok(rps_score)
}