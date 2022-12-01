#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};


pub fn get_max_calories(path: impl AsRef<Path>) -> Result<i64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let inp: Vec<i64> = br.lines().map(|x| x.unwrap().parse::<i64>().unwrap_or(-1)).collect::<Vec<_>>();

    let iter = inp.rsplit(|num| *num == -1);

    let calories: Vec<i64> = iter.map(|num| num.iter().sum::<i64>()).collect::<Vec<_>>();

    let max_cal: i64 = *calories.iter().max().unwrap();

    Ok(max_cal)
}


pub fn get_top3_calories(path: impl AsRef<Path>) -> Result<i64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let inp: Vec<i64> = br.lines().map(|x| x.unwrap().parse::<i64>().unwrap_or(-1)).collect::<Vec<_>>();

    let iter = inp.rsplit(|num| *num == -1);

    let mut calories: Vec<i64> = iter.map(|num| num.iter().sum::<i64>()).collect::<Vec<_>>();

    calories.sort();
    calories.reverse();
    calories.drain(3..);

    let top3_cal: i64 = calories.iter().sum();

    Ok(top3_cal)
}