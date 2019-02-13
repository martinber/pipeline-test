extern crate hound;

mod buf_map;

use buf_map::{Buffer, BufMapExt};

fn main() {

    // test()

    // bench()

    bench_buf()

}

fn test() {

    let result: Vec<f32> = (1..)
        .map(|x| x as f32)
        .map(|x| x * 2.)
        .buf_map(1, |buf| {
            buf[-1] * 2.
        })
        .buf_map(3, |buf| {
            let mut sum: f32 = 0_f32;
            for j in 0..3 {
                sum += buf[j] * 0.2;
            }
            sum
        })
        .take(100).collect();

    println!("{:?}", result);
}

fn bench() {

    let origin = (1..).map(|x| x as f32).take(100000000);

    let signal: Vec<f32> = origin.collect();


    let coeff = &[1., 1., 1.];
    let mut output: Vec<f32> = vec![0_f32; signal.len()];

    for i in 0..signal.len() {
        let mut sum: f32 = 0_f32;
        for j in 0..coeff.len() {
            if i > j {
                sum += signal.get(i - j).unwrap() * coeff[j];
            }
        }
        output[i] = sum;
    }


    // let c: Vec<f32> = b.collect();
}

fn bench_buf() {

    let a: Vec<f32> = (1..)
        .map(|x| x as f32)
        .take(100000000)
        .buf_map(3, |buf| filter(buf, &[1., 1., 1.]))
        .collect();
}

/// Filter a signal.
pub fn filter(
    buffer: &Buffer,
    coeff: &[f32],
) -> f32 {

    let mut sum: f32 = 0_f32;
    for j in 0..coeff.len() {
        sum += buffer[-(j as i32)] * coeff[j];
    }
    sum
}
