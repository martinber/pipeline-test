extern crate hound;


// https://gist.github.com/kevincox/019a0a4d1024e5bddd4be1cbe88fb2bc

use std::iter::Iterator;
use std::vec::IntoIter;
use std::collections::VecDeque;

// https://gist.github.com/kevincox/019a0a4d1024e5bddd4be1cbe88fb2bc
pub struct BufferedIterator {
    iter: IntoIter<f32>,
    buffer: Vec<f32>,
}

impl BufferedIterator {
    pub fn new(iter: IntoIter<f32>) -> Self {
        BufferedIterator{
            iter: iter,
            buffer: Vec::new(),
        }
    }

    pub fn peek(&mut self) -> Option<&f32> {
        if self.buffer.is_empty() {
            if let Some(e) = self.iter.next() {
                self.buffer.push(e)
            }
        }

        self.buffer.last()
    }

    pub fn unget(&mut self, e: f32) {
        self.buffer.insert(0, e)
    }
}

impl Iterator for BufferedIterator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.buffer.pop().or_else(|| self.iter.next())
    }
}

fn main() {

    let mut a = BufferedIterator::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.].into_iter());


    let b = duplicar(a);
    let c = duplicar(b);

    println!("{:?}", c.collect::<Vec<f32>>());


}

fn duplicar(entrada: impl Iterator<Item=f32>) -> impl Iterator<Item=f32> {

    entrada.map(|x| x * 2.)
}


/*
fn main() {

    let mut a = MyIter::new(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);


    let b = duplicar(a);
    let c = duplicar(b);

    println!("{:?}", c.collect::<Vec<f32>>());


}

fn duplicar(entrada: impl Iterator<Item=f32>) -> impl Iterator<Item=f32> {

    entrada.map(|x| x * 2.)
}

/// The front has the oldest value. The back has the newest value
struct MyIter {
    origin: Iterator<Item=f32>,
    buffer: VecDeque<f32>,
}

impl MyIter {
    pub fn new(v: Vec<f32>) -> MyIter {
        MyIter { origin: v.into_iter() }
    }

    pub fn get(&mut self, mut index: i32) -> &f32 {
        if index < 0 {
            // Negative indices count from back
            // index = match self.buffer.len().checked_add(index) {
                // Some(i) => i, // Return the index right away
                // None => { // len-index < 0, so we need to ask for more values
                    // panic!("End reached");
                // }
            // }

            // Ask for more values
            if self.buffer.len() as i32 + index < 0 {
                while self.buffer.len() as i32 + index < 0 {
                    self.buffer.push_front(self.origin.next());
                }
            }
            index = self.buffer.len() as i32 + index;
        }
        assert!(index >= 0);
        self.buffer.get(index as usize).unwrap()
    }
}


impl Iterator for MyIter {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.buffer.pop_back()
    }
}
*/


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
