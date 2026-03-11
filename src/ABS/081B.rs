use proconio::input;

fn main() {
    input!{
        n: i32,
        mut a: [i64; n],
    }
    let mut ans = 0;
    
//     loop {
//         // 各値が割り切れるかを先に確認
//         if a.iter().any(|&x| x % 2 != 0) {
//             break;
//         }
//         // 割り切れるなら割っていく
//         for x in &mut a {
//             *x /= 2;
//         }
//         ans += 1;
//     }

    // while a.iter().all(|&x| x % 2 == 0) {
    //     for x in &mut a {
    //         *x /= 2;
    //     }
    //     ans += 1;
    // }

    ans = a.iter().map(|x| x.trailing_zeros()).min().unwrap();
    println!("{}", ans);
}