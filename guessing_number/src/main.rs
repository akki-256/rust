use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("乱数を生成");
    let num = rand::thread_rng().gen_range(1..101);
    // println!("乱数は{}", num);

    loop {
        println!("数値を入力");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed read line");
        println!("You guessed : {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数値を入力");
                continue;
            }
        };

        match guess.cmp(&num) {
            Ordering::Equal => {
                println!("正解");
                break;
            }
            Ordering::Greater => println!("大きすぎます"),
            Ordering::Less => println!("小さすぎます"),
        }
    }
}
