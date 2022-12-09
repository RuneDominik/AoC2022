#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64
}

trait Movement {
    fn move_head(&mut self, movement: &str);
    fn move_tail(&mut self);
    fn update_visited(&mut self);
}

trait Count {
    fn count_visited(&self) -> u64;
}

#[derive(Debug, Clone)]
struct Rope {
    x_head: i64,
    y_head: i64,
    x_tail: i64,
    y_tail: i64,
    visited_points: Vec<Point>
}


impl Movement for Rope {
    fn move_head(&mut self, movement: &str) {
        let v: Vec<&str> = movement.split([' ']).collect();

        let dir: &str = v[0];
        let steps: u64 = v[1].parse::<u64>().unwrap();
    
        for _ in 0..(steps as usize) {
            match dir {
                "R" => self.x_head += 1,
                "L" => self.x_head -= 1,
                "U" => self.y_head += 1,
                "D" => self.y_head -= 1,
                _ => panic!()
            }

            self.move_tail();
        }
    }
    fn move_tail(&mut self) {
        let delta: f64 = f64::sqrt(((self.x_head-self.x_tail).pow(2) + (self.y_head-self.y_tail).pow(2)) as f64);

        if delta >= 1.5 { //sqrt(2) ~ 1.41, so this will do
            if self.y_head == self.y_tail {
                if self.x_head > self.x_tail {
                    self.x_tail += 1;
                } else {
                    self.x_tail -= 1;
                }
            } else if self.x_head == self.x_tail {
                if self.y_head > self.y_tail {
                    self.y_tail += 1;
                } else {
                    self.y_tail -= 1;
                }
            } else {
                if self.x_head > self.x_tail && self.y_head > self.y_tail {
                    self.x_tail += 1;
                    self.y_tail += 1;
                } else if self.x_head > self.x_tail && self.y_head < self.y_tail {
                    self.x_tail += 1;
                    self.y_tail -= 1;
                } else if self.x_head < self.x_tail && self.y_head < self.y_tail {
                    self.x_tail -= 1;
                    self.y_tail -= 1;
                } else {
                    self.x_tail -= 1;
                    self.y_tail += 1;
                }
            }
            self.update_visited();
        } 
    }
    fn update_visited(&mut self) {
        let new_tail_pos = Point { x:self.x_tail, y:self.y_tail };
        let already_visited = self.clone();

        if already_visited.visited_points.into_iter().find(|&x| x == new_tail_pos).is_none() {
            self.visited_points.push(new_tail_pos);
        }
    }
}


impl Count for Rope {
    fn count_visited(&self) -> u64 {
        return self.visited_points.len() as u64;
    }
}


pub fn get_n_visited(path: impl AsRef<Path>) -> Result<u64, Error> {
    let mut rope = Rope { x_head:0, y_head:0, x_tail:0, y_tail:0, visited_points:vec![Point {x:0, y:0}]};

    let file = File::open(path)?;

    let br = BufReader::new(file);

    for line in br.lines() {
        rope.move_head(&line.unwrap())
    }

    let n_visited = rope.count_visited();

    Ok(n_visited)
}