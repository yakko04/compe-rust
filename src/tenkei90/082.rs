use proconio::input;

fn main() {
    input! { l: i128, r: i128 }
    const D: i128 = 1_000_000_007;
    let mut ans: i128 = 0;

    let rtd = r.to_string().len() as i128 - 1;
    let ltd = l.to_string().len() as i128 - 1;

    for i in ltd..=rtd {
        let ilt = 10_i128.pow(i as u32);
        let irt = 10_i128.pow((i + 1) as u32) - 1;
        ans += (ilt + irt) * (irt - ilt + 1) / 2 * (i + 1);
        ans %= D;
    }

    ans -= (10_i128.pow(ltd as u32) + (l - 1)) * (l - 10_i128.pow(ltd as u32)) / 2 * (ltd + 1);
    ans -= ((r + 1) + (10_i128.pow((rtd + 1) as u32) - 1)) * (10_i128.pow((rtd + 1) as u32) - r - 1) / 2 * (rtd + 1);

    ans = ((ans % D) + D) % D;
    println!("{}", ans);
}
