use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n],
        b: [usize;n],
    }

    let mut diff = 0;
    for i in 0..n {
        diff += a[i].abs_diff(b[i]);
    }

    if diff <= k && (k - diff) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}