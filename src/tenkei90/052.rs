use proconio::input;

fn main() {
    const MOD: usize = 1_000_000_007;
    input!{
        n: usize,
        alist: [[usize; 6]; n],
    }

    let ans = alist.iter()
        .map(|row| row.iter().sum::<usize>() % MOD)
        .fold(1, |acc, s | acc * s % MOD);
    println!("{}", ans);
}