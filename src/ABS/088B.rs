use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }
    // let mut ans = 0;

    // a.sort();
    // a.reverse();
    // for i in 0..n {
    //     if i % 2 == 0 {
    //         ans += a[i];
    //     } else {
    //         ans -= a[i];
    //     }
    // }

    // println!("{}", ans);
    a.sort_by(|x,y| y.cmp(x));

    let ans : isize = a.iter()
        .enumerate()
        .map(|(i,&x)| if i % 2 == 0 { x } else { -x })
        .sum();
    println!("{}", ans);
}