use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize,usize);n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut class1 = vec![0; n + 1];
    let mut class2 = vec![0; n + 1];

    for i in 0..n {
        class1[i + 1] = class1[i];
        class2[i + 1] = class2[i];

        if cp[i].0 == 1 {
            class1[i + 1] += cp[i].1;
        } else {
            class2[i + 1] += cp[i].1;
        }
    }

    for (l, r) in lr {
        let sum1 = class1[r] - class1[l - 1];
        let sum2 = class2[r] -class2[l - 1];
        println!("{} {}", sum1, sum2);
    }

    // let mut f_acc = vec![0;n+1]; // first class accum
    // let mut s_acc = vec![0;n+1]; // second class accum

    // for i in 0..n {
    //     if cp[i].0 == 1 {
    //         f_acc[i+1] = f_acc[i] + cp[i].1;
    //         s_acc[i+1] = s_acc[i];
    //     } else {
    //         s_acc[i+1] = s_acc[i] + cp[i].1;
    //         f_acc[i+1] = f_acc[i];
    //     }
    // }

    // for i in 0..q {
    //     println!{"{} {}", f_acc[lr[i].1]-f_acc[lr[i].0 - 1], s_acc[lr[i].1]-s_acc[lr[i].0 - 1]}
    // }
}