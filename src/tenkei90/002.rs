use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if n % 2 == 1 {
        return ();
    }

    for i in 0..(1 << n) {
        let s: String = (0..n)
            .rev()
            .map(|bit| if (i >> bit) & 1 == 1 { ')'} else {'('})
            .collect();

        let mut depth = 0i32;
        let valid = s.chars().all(|c| {
            depth += if c == '(' {1} else {-1};
            depth >= 0
        }) && depth == 0;
        
        if valid {
            println!("{}", s);
        }
    }
}