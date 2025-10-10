use std::io;
fn main() {
    println!("数値を入力");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed read line");

    println!("You guessed : {}", guess);
}
