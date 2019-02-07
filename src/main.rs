extern crate hound;

fn main() {

    let mut a = MyIter::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);

    println!("{:?}", a.get(3));
    let b = duplicar(a);
    let c = duplicar(b);

    println!("{:?}", c.collect::<Vec<f32>>());


}

fn duplicar(entrada: impl std::iter::Iterator<Item=f32>) -> impl std::iter::Iterator<Item=f32> {

    entrada.map(|x| x * 2.)
}

struct MyIter {
    buffer: Vec<f32>,
}

impl MyIter {
    pub fn new(buffer: Vec<f32>) -> MyIter {
        MyIter { buffer }
    }

    pub fn get(&mut self, n: usize) -> &[f32] {
        &self.buffer[self.buffer.len() - n..]
    }
}

impl std::iter::Iterator for MyIter {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.buffer.pop()
    }
}


/*
/// Filter a signal.
pub fn filter(
    signal: impl std::iter::Iterator<Item=f32>,
    coeff: &[f32],
) -> impl std::iter::Iterator<Item=f32> {

    for i in 0..signal.len() {
        let mut sum: f32 = 0_f32;
        for j in 0..coeff.len() {
            if i > j {
                sum += signal[i - j] * coeff[j];
            }
        }
        output[i] = sum;
    }
    debug!("Filtering finished");

    context.step(Step::filter("filter_filter", &coeff))?;
    context.step(Step::signal("filter_result", &output, None))?;
    Ok(output)
}
*/
