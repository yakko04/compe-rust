use proconio::input;

fn solve(h: usize, w: usize) -> usize {
    if h == 1 || w == 1 {
        return h * w;
    } else {

        ((h + 1) / 2) * ((w + 1) / 2)
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    println!("{}", solve(h, w));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1_1() {
        assert_eq!(solve(1, 1), 1);
    }

    #[test]
    fn test_case_3_4() {
        assert_eq!(solve(3, 4), 4);
    }

    #[test]
    fn test_case_3_6() {
        assert_eq!(solve(3, 6), 6);
    }
}