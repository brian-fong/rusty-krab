fn find_largest_num(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

// fn find_largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     return largest;
// }

pub fn main() {
    println!();
    println!("+---------------+");
    println!("|   Duplicate   |");
    println!("+---------------+");
    let number_list = vec![34, 50, 25, 100, 65];
    let largest_num = find_largest_num(&number_list);
    println!("number_list: {:?}", number_list);
    println!("The largest number is {}", largest_num);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest_char =find_largest_char(&char_list);
    println!("char_list: {:?}", char_list);
    println!("The largest char is {}", largest_char);

    // println!();
    // println!("+-------------+");
    // println!("|   Generic   |");
    // println!("+-------------+");
    // let largest = find_largest(&number_list);
    // println!("The largest item is {}", largest);

    println!();
}
