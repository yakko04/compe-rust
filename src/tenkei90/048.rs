use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input!{
        n: usize,
        k: usize,
        ques: [(usize,usize); n]
    }

    let mut ab: BinaryHeap<(usize, usize)> = ques.into_iter().map(|(a,b)| (b, a-b)).collect();
    let mut ans = 0;

    for _ in 0..k {
        let (b, diff) = ab.pop().unwrap();
        ans += b;
        if diff > 0 {
            ab.push((diff, 0));
        }
    }

    println!("{}", ans);
    
}