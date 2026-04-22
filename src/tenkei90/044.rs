use proconio::input;

fn main() {
    input!{
        n: usize,
        q: usize,
        mut a: [usize;n],
        txy: [(usize,usize,usize);q]
    }

    // 現在の先頭が元のどこかを持つ
    let mut shift = 0;

    for (t,x,y) in txy {
        if t == 1 {
            let xi = (x - 1 + shift) % n;
            let yi = (y - 1 + shift) % n;
            a.swap(xi, yi);
        } else if t == 2 {
            shift = (shift + n - 1) % n;
        } else {
            let xi = (x - 1 + shift) % n;
            println!("{}", a[xi]);
        }
    }
}