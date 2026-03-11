use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        s: Chars,
    }
    let ans = s.iter().filter(|&&s| s =='1').count();
    println!("{}",ans);

}