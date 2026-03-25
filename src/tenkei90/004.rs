use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [usize; h * w],
    }

    let mut b = vec![vec![0; w]; h];
    let mut w_sum = vec![0; h]; // 各行の和
    let mut h_sum = vec![0; w]; // 各列の和

    for i in 0..h {
        for j in 0..w {
            w_sum[i] += a[i * w + j];
            h_sum[j] += a[i * w + j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            b[i][j] = w_sum[i] + h_sum[j] - a[i * w + j];
        }
    }

    for i in 0..h {
        println!("{}", b[i].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}