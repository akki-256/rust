use core::fmt;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use rand::prelude::*;

struct Numbers(Vec<i32>);

impl fmt::Display for Numbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for num in self.0.iter() {
            write!(f, "{}\n", num)?;
        }
        Ok(())
    }
}

fn push_unique_number(list: &Vec<i32>, many: &i32) -> i32 {
    let max = many * 2;
    let mut rng = rand::rng();
    let mut rand_num: i32;
    let unique_num = loop {
        rand_num = rng.random_range(0..max);
        match list.iter().find(|&&item| item == rand_num) {
            Some(_) => continue,
            None => break rand_num,
        };
    };
    unique_num
}

fn create_ramdam_numbers(many: &i32) -> Vec<i32> {
    let mut numbers = Vec::new();
    for _ in 0..*many {
        numbers.push(push_unique_number(&numbers, &many));
    }
    numbers
}

fn main() {
    let path = "./data.txt";
    let file = OpenOptions::new().write(true).create(true).open(path);

    let file = match file {
        Ok(f) => f,
        Err(_) => {
            panic!("ファイルの作成に失敗しました");
        }
    };
    let mut buf_writer = BufWriter::new(file);
    let numbers = create_ramdam_numbers(&100);
    let numbers = Numbers(numbers);
    writeln!(buf_writer, "{}", numbers).unwrap();
    buf_writer.flush().unwrap();
}
