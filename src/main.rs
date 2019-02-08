extern crate hound;


// https://gist.github.com/kevincox/019a0a4d1024e5bddd4be1cbe88fb2bc

use std::iter::Iterator;
use std::collections::VecDeque;

fn main() {

    // test()

    // bench()

    bench_iter()

}

fn test() {

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

fn bench_iter() {

    let origin = (1..).map(|x| x as f32).take(100000000);


    let b = MyIter::new(origin, 3, |buf| filter_2(buf, &[1., 1., 1.]));

    let c: Vec<f32> = b.collect();
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

    pub fn get_neg(&self, mut index: usize) -> f32 {
        index = self.deque.len().checked_sub(index + 1).unwrap();
        *self.deque.get(index)
            .expect("Out of bounds")
    }

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

    /// Pull a value from the iterator. Returns if able to get value.
    pub fn pull(&mut self) -> bool {
        match self.origin.next() {
            Some(v) => {
                self.deque.push_front(v);
                true
            },
            None => false,
        }
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
        let mut iter = MyIter {
            buffer: Buffer::new(iter),
            function,
            // function: |buf: Buffer<Iter>| buf.deque.pop_back(),
            window,
        };
        // Because I pull before next()
        if ! iter.buffer.fill(window - 1) {
            panic!("AA");
        }
        iter
    }
}



impl<F> Iterator for MyIter<F>
    where F: Fn(&Buffer) -> f32
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.buffer.pull() {
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

/// Filter a signal.
pub fn filter_2(
    buffer: &Buffer,
    coeff: &[f32],
) -> f32 {

    let mut sum: f32 = 0_f32;
    for j in 0..coeff.len() {
        sum += buffer.get_neg(j) * coeff[j];
    }
    sum
}
