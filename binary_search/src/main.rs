use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn binary_search(list: &[i32], current: i32) -> Option<usize> {
    let mut current_index = list.len() / 2;
    let mut herf = current_index / 2;

    while list[current_index] != current {
        if herf == 0 {
            return None;
        }

        if list[current_index] < current {
            //対象が大きい場合
            current_index = current_index + herf;
        } else {
            //小さい場合
            current_index = current_index - herf;
        }

        herf /= 2;
    }

    Some(current_index)
}

fn main() {
    let f = File::open("./data.txt").expect("ファイル読み込みエラー");
    let buf = BufReader::new(f);
    let mut numbers = Vec::new();

    for input in buf.lines() {
        match input {
            Err(e) => {
                panic!("{}", e)
            }
            Ok(line) => {
                numbers.push(line.parse::<i32>().expect("数値が入力されていません"));
            }
        }
    }

    numbers.sort();
    let mut numbers: &[i32] = numbers.as_slice();

    match binary_search(&mut numbers, 2) {
        Some(index) => {
            println!("found!! index:{}", index);
        }
        None => {
            println!("Not found");
        }
    }
}
