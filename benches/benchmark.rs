#[macro_use]
extern crate criterion;
extern crate pipeline_test;

use criterion::{Criterion, ParameterizedBenchmark};
use pipeline_test::*;
use std::iter::Iterator;

pub fn buffer_filter<I>(
    buffer: &Buffer<I>,
    coeff: &[f32],
) -> f32
    where I: Iterator<Item=f32> {

    let mut sum: f32 = 0_f32;
    for j in 0..coeff.len() {
        sum += buffer[-(j as i32)] * coeff[j];
    }
    sum
}

pub fn slow_buffer_filter(
    buffer: &SlowBuffer,
    coeff: &[f32],
) -> f32 {

    let mut sum: f32 = 0_f32;
    for j in 0..coeff.len() {
        sum += buffer[-(j as i32)] * coeff[j];
    }
    sum
}

pub fn loop_filter(
    input: &Vec<f32>,
    coeff: &[f32],
) -> Vec<f32> {

    let mut output: Vec<f32> = vec![0_f32; input.len()];

    for i in 0..input.len() {
        let mut sum: f32 = 0_f32;
        for j in 0..coeff.len() {
            if i > j {
                sum += input[i - j] * coeff[j];
            }
        }
        output[i] = sum;
    }
    output
}

const COEFFS_100: [f32; 100] = [
    1., 2., 3., 4., 5., 6., 7., 8., 9., 10.,
    11., 12., 13., 14., 15., 16., 17., 18., 19., 20.,
    1., 2., 3., 4., 5., 6., 7., 8., 9., 10.,
    11., 12., 13., 14., 15., 16., 17., 18., 19., 20.,
    1., 2., 3., 4., 5., 6., 7., 8., 9., 10.,
    11., 12., 13., 14., 15., 16., 17., 18., 19., 20.,
    1., 2., 3., 4., 5., 6., 7., 8., 9., 10.,
    11., 12., 13., 14., 15., 16., 17., 18., 19., 20.,
    1., 2., 3., 4., 5., 6., 7., 8., 9., 10.,
    11., 12., 13., 14., 15., 16., 17., 18., 19., 20.,
];

const COEFFS_20: [f32; 20] = [
    1., 0., 2., 0., 3., 0., 4., 0., 5., 0.,
    1., 0., 2., 0., 3., 0., 4., 0., 5., 0.,
];

fn buf_map_bench(c: &mut Criterion) {

    let vec_sizes = vec![1000, 5000, 10000];

    // c.bench_function("buf_map", |b| b.iter( ||{
        // let a: Vec<f32> = (1..)
            // .map(|x| x as f32)
            // .take(10000)
            // .buf_map(3, |buf| buffer_filter(buf, &[1., 1., 1.]))
            // .buf_map(100, |buf| buffer_filter(buf, &COEFFS_100))
            // .buf_map(20, |buf| buffer_filter(buf, &COEFFS_20))
            // .collect();
    // }));
//
    // c.bench_function("iterate_vec", |b| b.iter( ||{
        // let a: Vec<f32> = (1..)
            // .map(|x| x as f32)
            // .take(10000)
            // .collect();
//
        // let a: Vec<f32> = loop_filter(&a, &[1., 1., 1.]);
        // let a: Vec<f32> = loop_filter(&a, &COEFFS_100);
        // let a: Vec<f32> = loop_filter(&a, &COEFFS_20);
    // }));

    c.bench(
        "Maps",
        ParameterizedBenchmark::new("buf_map", |b, i| b.iter(|| {
            let a: Vec<f32> = (1..)
                .map(|x| x as f32)
                .take(*i)
                .buf_map(3, |buf| buffer_filter(buf, &[1., 1., 1.]))
                .buf_map(100, |buf| buffer_filter(buf, &COEFFS_100))
                .buf_map(20, |buf| buffer_filter(buf, &COEFFS_20))
                .collect();
        }), vec_sizes)
        .with_function("slow_buf_map", |b, i| b.iter(|| {
            let a: Vec<f32> = (1..)
                .map(|x| x as f32)
                .take(*i)
                .slow_buf_map(3, |buf| slow_buffer_filter(buf, &[1., 1., 1.]))
                .slow_buf_map(100, |buf| slow_buffer_filter(buf, &COEFFS_100))
                .slow_buf_map(20, |buf| slow_buffer_filter(buf, &COEFFS_20))
                .collect();
        }))
        .with_function("Iterative", |b, i| b.iter(|| {
            let a: Vec<f32> = (1..)
                .map(|x| x as f32)
                .take(*i)
                .collect();

            let a: Vec<f32> = loop_filter(&a, &[1., 1., 1.]);
            let a: Vec<f32> = loop_filter(&a, &COEFFS_100);
            let a: Vec<f32> = loop_filter(&a, &COEFFS_20);
        })),
    );
}

criterion_group!(benches, buf_map_bench);
criterion_main!(benches);
