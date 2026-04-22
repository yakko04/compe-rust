use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut am = [0usize; 46];
    for i in 0..n { am[a[i] % 46] += 1; }

    let mut bm = [0usize; 46];
    for i in 0..n { bm[b[i] % 46] += 1; }

    let mut cm = [0usize; 46];
    for i in 0..n { cm[c[i] % 46] += 1; }

    let mut ans = 0;
    for av in 0..46 {
        for bv in 0..46 {
            let cv = (46 - (av + bv) % 46) % 46;
            ans += am[av] * bm[bv] * cm[cv];
        }
    }

    println!("{}", ans);
}