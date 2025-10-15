// fn bubble_upper(list: &mut [i32]) {
//     for i in 0..list.len() {
//         for j in 0..list.len() - i - 1 {
//             if list[j] > list[j + 1] {
//                 list.swap(j, j + 1);
//             }
//         }
//     }
// }

// fn bubble_lower(list: &mut [i32]) {
//     for i in 0..list.len() {
//         for j in 0..list.len() - i - 1 {
//             if list[j] < list[j + 1] {
//                 list.swap(j, j + 1);
//             }
//         }
//     }
// }

fn quick_sort(list: &mut [i32]) {
    // let mut i = String::new();
    // std::io::stdin().read_line(&mut i).expect("msg");

    if list.len() <= 1 {
        return;
    }

    // //ピポッド決定
    let pipod: i32 = list.iter().sum();
    let pipod = pipod as f32 / list.len() as f32;

    loop {
        //前方ピポッド以上探索
        let mut over_pipod = 0;
        for index in 0..list.len() - 1 {
            if pipod <= list[index] as f32 {
                over_pipod = index;
                break;
            }
        }

        //後方ピポッド以下探索
        let mut lower_pipod = list.len() - 1;
        for index in (0..list.len()).rev() {
            if pipod > list[index] as f32 {
                lower_pipod = index;
                break;
            }
        }

        if over_pipod < lower_pipod {
            list.swap(over_pipod, lower_pipod);
        } else {
            let (front_list, end_list) = list.split_at_mut(lower_pipod + 1);
            quick_sort(front_list);
            quick_sort(end_list);
            break;
        }
    }
}

fn main() {
    let mut list = [2, 4, 3, 10, 5, 7, 1, 0, 8];
    quick_sort(&mut list);
    println!("{:?}", list);
}
