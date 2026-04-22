use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q],
    }

    for ei in e {
        // 時刻eiでの角度（下が0、時計回り）
        let theta = 2.0 * PI * ei / t;
        // 円の中心は(0, 0, L/2)、半径L/2で回転
        // let cx = 0.0;
        let cy = - l / 2.0 * theta.sin();
        let cz = l / 2.0 - l / 2.0 * theta.cos();

        let dy = cy - y;

        let dh = (x * x + dy * dy).sqrt();
        let angle = cz.atan2(dh).to_degrees();

        println!("{:.10}", angle);
    }
}