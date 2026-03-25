use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map: HashMap<String, usize> = HashMap::new();

    for i in 0..n {
        if !map.contains_key(&s[i]) {
            println!("{}", i + 1);
        } 
        map.entry(s[i].clone()).or_insert(1);
    }
}