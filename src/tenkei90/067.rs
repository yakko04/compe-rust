use proconio::input;

fn oct_todecimal(mut oct: u128) -> u128 {
    let mut ans = 0;
    let mut digit = 0;

    while oct != 0 {
        ans += (oct % 10) * 8_u128.pow(digit);
        digit += 1;
        oct /= 10;
    }
    ans
}

fn dec_tonona(mut dec: u128) -> u128 {
    let mut ans = 0;
    let mut digit = 0;

    while dec != 0 {
        ans += (dec % 9) * 10_u128.pow(digit);
        digit += 1;
        dec /= 9
    }
    ans
}

fn transform_to5(num: u128) -> u128 {
    num.to_string().replace('8', "5").parse().unwrap()
}
fn main() {
    input!{
        mut n: u128,
        k: u128,
    }

    for _ in 0..k {
        n =transform_to5(dec_tonona(oct_todecimal(n)));
    }

    println!("{}", n);
}