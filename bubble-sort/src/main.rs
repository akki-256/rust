fn bubble_upper(list: &mut [i32]) {
    for i in 0..list.len() {
        for j in 0..list.len() - i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}

fn bubble_lower(list: &mut [i32]) {
    for i in 0..list.len() {
        for j in 0..list.len() - i - 1 {
            if list[j] < list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut list = [2, 4, 3, 10, 5, 7, 1, 4, 3, 0, 8];
    println!("{:?}", list);
    bubble_upper(&mut list);
    println!("{:?}", list);
    bubble_lower(&mut list);
    println!("{:?}", list);
}
