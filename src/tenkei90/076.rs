use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }
    let target = a.iter().sum::<usize>();
    let double = a.repeat(2);

    let mut now = 0;
    let mut left = 0;

    for right in 0..n*2 {
        now += double[right];
        while now * 10 > target {
            now -= double[left];
            left += 1;
        }
        if now * 10 == target {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");

}
