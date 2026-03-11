use proconio::input;

fn main() {
    input!{
        s: String,
    }

    let words = ["dream", "dreamer", "erase", "eraser"];
    let mut s = s.as_str();

    while !s.is_empty() {
        if let Some(rest) = words.iter().find_map(|&w| s.strip_suffix(w)) {
            s = rest;
        } else {
            println!("NO");
            return;
        }
    }

    println!("YES");
}


// fn main() {
//     input!{
//         s: String,
//     }
//     let length = s.len();
//     let t = ["dream", "dreamer", "erase", "eraser"];
//     let mut index = 0;
//     let reversed: String = s.chars().rev().collect();

//     while index < length {
//         let mut flag = false;
//         for i in 0..4 {
//             let check: String = t[i].chars().rev().collect();
//             let end = min(index + check.len(), length);
//             if end - index == check.len() && reversed[index..end] == check {
//                 index += check.len();
//                 flag = true;
//                 break;
//             }
//         }
//         if !flag {
//             println!("NO");
//             return;
//         }
//     }

//     println!("YES");
// }