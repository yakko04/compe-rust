use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        query: [(usize, usize, i64); q],
    }

    let mut b = vec![0i64; n - 1];
    for i in 0..n - 1 {
        b[i] = a[i + 1] - a[i];
    }

    let mut total: i64 = b.iter().map(|x| x.abs()).sum();

    for (l, r, v) in query {
        let li = l - 1;
        if li > 0 {
            total -= b[li - 1].abs();
            b[li - 1] += v;
            total += b[li - 1].abs();
        }
        if r < n {
            total -= b[r - 1].abs();
            b[r - 1] -= v;
            total += b[r - 1].abs();
        }
        println!("{}", total);
    }
}
