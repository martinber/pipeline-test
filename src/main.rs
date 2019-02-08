extern crate hound;


// https://gist.github.com/kevincox/019a0a4d1024e5bddd4be1cbe88fb2bc

use std::iter::Iterator;
use std::collections::VecDeque;


fn main() {

    let mut a = MyIter::new(
        vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.].into_iter(),
        5,
        |buf| buf.deque.pop_back().or(Some(0.)).unwrap()
    );

    // println!("{:?}", a.get(-1));

    let b = duplicar(a);
    let c = duplicar(b).take(10);

    println!("{:?}", c.collect::<Vec<f32>>());


}

fn duplicar(entrada: impl Iterator<Item=f32>) -> impl Iterator<Item=f32> {

    entrada.map(|x| x * 2.)
}


struct Buffer<Iter>
    where Iter: Iterator<Item=f32> + Sized
{
    origin: Iter,
    pub deque: VecDeque<f32>,
}

impl<Iter> Buffer<Iter>
    where Iter: Iterator<Item=f32> + Sized
{
    pub fn new(v: Iter) -> Buffer<Iter> {
        Buffer {
            origin: v,
            deque: VecDeque::new(),
        }
    }

    pub fn get(&mut self, mut index: i32) -> &f32 {
        if index < 0 {

            // Ask for more values
            if self.deque.len() as i32 + index < 0 {
                while self.deque.len() as i32 + index < 0 {
                    self.deque.push_front(self.origin.next().unwrap());
                }
            }
            index = self.deque.len() as i32 + index;
        }
        assert!(index >= 0);
        println!("{:?}, {}", self.deque, index);
        self.deque.get(index as usize).unwrap()
    }

    /// Fills the buffer to reach given length. Returns if succesful.
    pub fn fill(&mut self, length: usize) -> bool {
        while self.deque.len() < length {
            match self.origin.next() {
                Some(v) => self.deque.push_front(v),
                None => return false,
            }
        }
        return true;
    }
}


/// The front has the oldest value. The back has the newest value
struct MyIter<Iter, F>
    where Iter: Iterator<Item=f32> + Sized,
          F: Fn(&mut Buffer<Iter>) -> f32
{
    buffer: Buffer<Iter>,
    function: F,
    window: usize,
}

impl<Iter, F> MyIter<Iter, F>
    where Iter: Iterator<Item=f32> + Sized,
          F: Fn(&mut Buffer<Iter>) -> f32
{
    pub fn new(iter: Iter, window: usize, function: F) -> MyIter<Iter, F>
    {
        MyIter {
            buffer: Buffer::new(iter),
            function,
            // function: |buf: Buffer<Iter>| buf.deque.pop_back(),
            window,
        }
    }
}



impl<Iter, F> Iterator for MyIter<Iter, F>
    where Iter: Iterator<Item=f32> + Sized,
          F: Fn(&mut Buffer<Iter>) -> f32
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // self.buffer.pop_back().or_else(|| self.origin.next())
        if self.buffer.fill(self.window) {
            Some((self.function)(&mut self.buffer))
        } else {
            None
        }
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
