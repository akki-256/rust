use std::io;

fn main() {
    println!("フィボナッチ数列を出力します");
    println!("数値を入力");

    let mut max_num = String::new();
    io::stdin()
        .read_line(&mut max_num)
        .expect("入力ができませんでした");

    let max_num: u128 = match max_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("数値を入力してください");
            return;
        }
    };

    let mut buck_num = 0;
    let mut num = 1;
    let mut next_num = 1;
    print!("{}", buck_num);
    loop {
        buck_num = num;
        num = next_num;
        next_num = num + buck_num;
        print!(",{}", num);
        if next_num > max_num {
            break;
        }
    }
    println!();
}
