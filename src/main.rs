extern crate hound;


// https://gist.github.com/kevincox/019a0a4d1024e5bddd4be1cbe88fb2bc

use std::iter::Iterator;
use std::collections::VecDeque;

type SignalIter = Iterator<Item=f32>;

fn main() {

    let origin = (1..).map(|x| x as f32);

    let a = MyIter::new(origin, 1, |buf| {
        buf.get(-1)// * 2.
    });

    let b = MyIter::new(a, 3, |buf| filter(buf, &[1., 1., 1.]));

    let c = MyIter::new(b, 1, |buf| {
        buf.get(-1)// + 10.
    });

    /*
    let a = origin.map(|x| {
        x * 2.
    });

    let b = a.map(|x| {
        x + 10.
    });
    */

    // println!("{:?}", a.get(-1));

    // let d: Vec<f32> = c.take(1000000000).collect();

    let d: Vec<f32> = c.take(100).collect();
    println!("{:?}", d);


}

fn duplicar(entrada: impl Iterator<Item=f32>) -> impl Iterator<Item=f32> {

    entrada.map(|x| x * 2.)
}


pub struct Buffer
{
    origin: Box<Iterator<Item=f32>>,
    pub deque: VecDeque<f32>,
}

impl Buffer
{
    pub fn new(v: impl Iterator<Item=f32> + 'static) -> Buffer {
        Buffer {
            origin: Box::new(v),
            deque: VecDeque::new(),
        }
    }

    // pub fn get(&mut self, mut index: i32) -> f32 {
        // if index < 0 {
//
            // // Ask for more values
            // if self.deque.len() as i32 + index < 0 {
                // while self.deque.len() as i32 + index < 0 {
                    // self.deque.push_front(self.origin.next().unwrap());
                // }
            // }
            // index = self.deque.len() as i32 + index;
        // }
        // assert!(index >= 0);
        // *self.deque.get(index as usize).unwrap()
    // }

    pub fn get(&self, mut index: i32) -> f32 {
        if index < 0 {
            index = self.deque.len() as i32 + index;
        }
        if index < 0 {
            panic!("Negative buffer index out of bounds");
        }
        *self.deque.get(index as usize)
            .expect("Positive buffer index out of bounds")
    }

    pub fn iter(&self) -> impl Iterator<Item=&f32> + '_ {
        self.deque.iter().rev()
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

    pub fn pop(&mut self) -> Option<f32> {
        self.deque.pop_back()
    }
}


/// The front has the oldest value. The back has the newest value
struct MyIter<F>
    where F: Fn(&Buffer) -> f32
{
    buffer: Buffer,
    function: F,
    window: usize,
}

impl<F> MyIter<F>
    where F: Fn(&Buffer) -> f32
{
    pub fn new(iter: impl Iterator<Item=f32> + 'static, window: usize, function: F) -> MyIter<F>
    {
        MyIter {
            buffer: Buffer::new(iter),
            function,
            // function: |buf: Buffer<Iter>| buf.deque.pop_back(),
            window,
        }
    }
}



impl<F> Iterator for MyIter<F>
    where F: Fn(&Buffer) -> f32
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // self.buffer.pop_back().or_else(|| self.origin.next())
        if self.buffer.fill(self.window) {
            let result = (self.function)(&self.buffer);
            self.buffer.pop();
            Some(result)
        } else {
            None
        }
    }
}


/// Filter a signal.
pub fn filter(
    buffer: &Buffer,
    coeff: &[f32],
) -> f32 {

    let mut sum: f32 = 0_f32;
    for j in 0..coeff.len() {
        sum += buffer.get(-(j as i32)) * coeff[j];
    }
    sum
}
