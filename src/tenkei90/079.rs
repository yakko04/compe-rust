use proconio::input;

fn main() {
    input!{
        h: usize,
        w: usize,
        mut a: [[isize; w]; h],
        b: [[isize; w]; h],
    }
    let mut cnt: isize = 0;

    for i in 0..h-1 {
        for j in 0..w-1 {
            let diff: isize = a[i][j] - b[i][j];
            if diff != 0 {
                cnt += diff.abs();
                a[i][j] -= diff;
                a[i+1][j] -= diff;
                a[i][j+1] -= diff;
                a[i+1][j+1] -= diff;
            }
        }
    }

    if a == b {
        println!("Yes");
        println!("{}", cnt);
    } else {
        println!("No");
    }

}
