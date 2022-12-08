#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};


fn is_visible_left(heights: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize, x_len: usize, y_len: usize, first_height: u8) -> bool {
    if pos_x == 0 {
        return true;
    }
    
    let cond: bool = first_height > heights[pos_y][pos_x-1];

    if cond && is_visible_left(heights, pos_x-1, pos_y, x_len, y_len, first_height) {
        return true;
    } else {
        return false;
    }
}

fn is_visible_top(heights: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize, x_len: usize, y_len: usize, first_height: u8) -> bool {
    if pos_y == 0 {
        return true;
    } 
    
    let cond: bool;
    cond = first_height > heights[pos_y-1][pos_x];

    if cond && is_visible_top(heights, pos_x, pos_y-1, x_len, y_len, first_height) {
        return true;
    } else {
        return false;
    }
}

fn is_visible_right(heights: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize, x_len: usize, y_len: usize, first_height: u8) -> bool {
    if pos_x == x_len-1 {
        return true;
    }

    let cond: bool;

    cond = first_height > heights[pos_y][pos_x+1];
    
    if cond && is_visible_right(heights, pos_x+1, pos_y, x_len, y_len, first_height) {
        return true;
    } else {
        return false;
    }
}

fn is_visible_bottom(heights: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize, x_len: usize, y_len: usize, first_height: u8) -> bool {
    if pos_y == y_len-1 {
        return true;
    }
    
    let cond: bool = first_height > heights[pos_y+1][pos_x];
    
    if cond && is_visible_bottom(heights, pos_x, pos_y+1, x_len, y_len, first_height) {
        return true;
    } else {
        return false;
    }
}


fn is_visible(heights: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize, x_len: usize, y_len: usize) -> bool {
    let current_height = heights[pos_y][pos_x];

    let vis_left: bool = is_visible_left(heights, pos_x, pos_y, x_len, y_len, current_height);
    let vis_top: bool = is_visible_top(heights, pos_x, pos_y, x_len, y_len, current_height);
    let vis_right: bool = is_visible_right(heights, pos_x, pos_y, x_len, y_len, current_height);
    let vis_bottom: bool = is_visible_bottom(heights, pos_x, pos_y, x_len, y_len, current_height);

    let vis: bool = vis_left || vis_top || vis_right || vis_bottom;
    return vis;
}


fn look_left(heights: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize, x_len: usize, y_len: usize, first_height: u8) -> u64 {
    if pos_x == 0 {
        return 0;
    }
    
    let cond: bool = first_height > heights[pos_y][pos_x-1];
    
    if cond {
        return look_left(heights, pos_x-1, pos_y, x_len, y_len, first_height)+1;
    } else {
        return 1;
    }
}

fn look_up(heights: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize, x_len: usize, y_len: usize, first_height: u8) -> u64 {
    if pos_y == 0 {
        return 0;
    }
    
    let cond: bool = first_height > heights[pos_y-1][pos_x];
    
    if cond {
        return look_up(heights, pos_x, pos_y-1, x_len, y_len, first_height)+1;
    } else {
        return 1;
    }
}

fn look_right(heights: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize, x_len: usize, y_len: usize, first_height: u8) -> u64 {
    if pos_x == x_len-1 {
        return 0;
    }
    
    let cond: bool = first_height > heights[pos_y][pos_x+1];
    
    if cond {
        return look_right(heights, pos_x+1, pos_y, x_len, y_len, first_height)+1;
    } else {
        return 1;
    }
}

fn look_down(heights: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize, x_len: usize, y_len: usize, first_height: u8) -> u64 {
    if pos_y == y_len-1 {
        return 0;
    }
    
    let cond: bool = first_height > heights[pos_y+1][pos_x];
    
    if cond {
        return look_down(heights, pos_x, pos_y+1, x_len, y_len, first_height)+1;
    } else {
        return 1;
    }
}


fn score(heights: &Vec<Vec<u8>>, pos_x: usize, pos_y: usize, x_len: usize, y_len: usize) -> u64 {
    let current_height = heights[pos_y][pos_x];

    let score_left: u64 = look_left(heights, pos_x, pos_y, x_len, y_len, current_height);
    let score_up: u64 = look_up(heights, pos_x, pos_y, x_len, y_len, current_height);
    let score_right: u64 = look_right(heights, pos_x, pos_y, x_len, y_len, current_height);
    let score_bottom: u64 = look_down(heights, pos_x, pos_y, x_len, y_len, current_height);

    let vis: u64 = score_left * score_up * score_right * score_bottom;
    return vis;
}


pub fn get_n_visible(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let mut heights = Vec::new();
    let mut mask = Vec::new();
    for line in br.lines() {
        let row: Vec<u8> = line.unwrap()
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();
        mask.push(vec![0; row.len()]);
        heights.push(row);
    }

    let x_len: usize = heights[0].len();
    let y_len: usize = heights.len();

    for y_pos in 0..y_len {
        for x_pos in 0..x_len {
            if is_visible(&heights, x_pos, y_pos, x_len, y_len) {
                mask[y_pos][x_pos] = 1
            }
        }
    }

    let n_visible: u64 = mask.into_iter().map(|x| x.into_iter().sum::<u64>()).sum();

    Ok(n_visible)
}

pub fn get_scenic_score(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let mut heights = Vec::new();
    let mut mask: Vec<Vec<u64>> = Vec::new();
    for line in br.lines() {
        let row: Vec<u8> = line.unwrap()
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();
        mask.push(vec![0; row.len()]);
        heights.push(row);
    }

    let x_len: usize = heights[0].len();
    let y_len: usize = heights.len();

    for y_pos in 0..y_len {
        for x_pos in 0..x_len {
            mask[y_pos][x_pos] = score(&heights, x_pos, y_pos, x_len, y_len)
        }
    }

    let max_score: u64 = mask.into_iter().flatten().collect::<Vec<u64>>().into_iter().max().unwrap();

    Ok(max_score)
}