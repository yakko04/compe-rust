use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        mut b: [usize; q],
    }

    a.sort();

    for &bi in &b {
        let pos = a.partition_point(|&x| x < bi);
        let mut best = usize::MAX;

        // 最大対象レーティングを超えない場合
        if pos < n {
            best = best.min(a[pos].abs_diff(bi));
        }
        //　最小レーティングを下回らない場合
        if pos > 0 {
            best = best.min(a[pos - 1].abs_diff(bi));
        }

        println!("{}", best);
    }

}