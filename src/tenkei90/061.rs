use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q]
    }

    let mut deq = VecDeque::new();

    for (t, x) in tx {
        match t {
            1 => deq.push_front(x),
            2 => deq.push_back(x),
            3 => println!("{}", deq[x - 1] ),
            _ => unreachable!()
        }
    }
}