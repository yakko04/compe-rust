use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    }
    let mut ans = usize::MAX;
    for perm in (0..n).permutations(n) {
        let mut ok = true;
        for &(x, y) in &xy {
            if perm.windows(2).any(|w| (w[0] == x - 1 && w[1] == y - 1) || (w[0] == y - 1 && w[1] == x - 1)) {
                ok = false;
                break;
            }
        }
        if !ok { continue; }
        let cost: usize = perm.iter().enumerate().map(|(i, &p)| a[p][i]).sum();
        ans = ans.min(cost);
    }
    println!("{}", if ans == usize::MAX { -1 } else { ans as i64 });
}