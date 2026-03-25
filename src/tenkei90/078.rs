use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(usize,usize); m]
    }

    let mut adj = vec![vec![]; n];

    for (a,b) in ab {
        if a > b {
            adj[a - 1].push(b-1);
        } else {
            adj[b - 1].push(a-1);
        }
    }
    
    println!("{}", adj.iter().filter(|v| v.len() == 1).count())

}