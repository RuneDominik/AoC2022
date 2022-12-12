#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};


#[derive(Debug)]
struct Register<'a > {
    i: u64,
    x: i64,
    current_op: i64,
    current_op_timer: i64,
    drawing: Vec<Vec<&'a str>>
}

trait Progress {
    fn cycle(&mut self, op: &str, sigstr: &mut i64);
    fn add_op(&mut self, op: &str);
    fn exec_op(&mut self);
    fn draw(&mut self);
}

trait Show {
    fn show(&mut self);
}

impl Show for Register<'_> {
    fn show(&mut self) {
        println!("{:?}", self.drawing[0]);
        println!("{:?}", self.drawing[1]);
        println!("{:?}", self.drawing[2]);
        println!("{:?}", self.drawing[3]);
        println!("{:?}", self.drawing[4]);
        println!("{:?}", self.drawing[5]);
    }
}

impl Progress for Register<'_> {
    fn add_op(&mut self, op: &str) {
        if self.current_op_timer != 0 {
            panic!();
        }
        if op == "noop" {
            self.current_op = 0;
            self.current_op_timer = 1;
        } else {
            let op_vec: Vec<&str> = op.split(' ').collect();
            self.current_op = op_vec[1].parse::<i64>().unwrap();
            self.current_op_timer = 2;
        }
    }
    fn exec_op(&mut self) {
        if self.current_op_timer == 0 {
            self.x += self.current_op;
            self.current_op = 0;
        }
    }
    fn draw(&mut self){
        let current_row = ((self.i - 1) / 40) as usize;

        if current_row < 6 {
            if self.x - 1 <= ((self.i - 1) % 40) as i64 && ((self.i - 1) % 40) as i64 <= self.x +1 {
                self.drawing[current_row].push("#");
            } else {
                self.drawing[current_row].push(".");
            }
        }
    }
    fn cycle(&mut self, op: &str, sigstr: &mut i64) {
        self.add_op(op);
        while self.current_op_timer != 0 {
            self.i += 1;
            self.current_op_timer -= 1;
            if ((self.i+20) % 40) == 0 && self.i < 222 {
                *sigstr += self.i as i64 * self.x;
            }
            self.draw();
            self.exec_op();
        }
    }
}

pub fn get_sig_strength(path: impl AsRef<Path>) -> Result<i64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);

    let mut register = Register {
        i:0, 
        x:1, 
        current_op:0, 
        current_op_timer:0, 
        drawing:vec![Vec::<&str>::new(), Vec::<&str>::new(), Vec::<&str>::new(), Vec::<&str>::new(), Vec::<&str>::new(), Vec::<&str>::new()]
    };

    let mut signal_strength: i64 = 0;

    for line in br.lines() {
        register.cycle(&line.unwrap(), &mut signal_strength);
    }

    register.show();

    Ok(signal_strength)
}