use proconio::input;

fn main() {
    input!{
        n: usize,
        txy: [(isize,isize,isize); n],
    }
    let mut prev = (0,0,0);
    
    for (t,x,y) in txy {
        let dt = t - prev.0;
        let dist = (x - prev.1).abs() + (y - prev.2).abs();

        if dist > dt || (dt - dist) % 2 == 1 {
            println!("No");
            return;
        }

        prev = (t, x, y);
    }

    println!("Yes");



}