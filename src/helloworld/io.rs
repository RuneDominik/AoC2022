#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};

pub fn get_msg(path: impl AsRef<Path>) -> Result<String, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let msg: String = br
        .lines()
        .next()
        .unwrap()
        .unwrap();


    Ok(msg)
}