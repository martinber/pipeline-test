extern crate hound;

use std::iter::Iterator;
use std::collections::VecDeque;

/// An iterator that maps the values of an `Iterator` using a `VecDeque` as a
/// buffer.
///
/// Usage: `Iterator.buf_map(window_size, function)`
/// Example: `(1..).buf_map(3, |buf| buf[0] + buf[1] + buf[2])`
pub struct BufMap<F, I>
    where F: Fn(&VecDeque<f32>) -> f32,
          I: Iterator<Item=f32>
{
    buffer: VecDeque<f32>,
    source: I,
    function: F,
}

impl<F, I> BufMap<F, I>
    where F: Fn(&VecDeque<f32>) -> f32,
          I: Iterator<Item=f32>
{
    pub fn new(source: I, window: usize, function: F) -> BufMap<F, I>
    {
        let mut iter = BufMap {
            buffer: VecDeque::with_capacity(window + 1),
            source,
            function,
        };
        // Pull values from the source iterator until there is only one empty
        // place on the buffer.
        // Leave one place empty because I'm pulling a values from the iterator
        // when `next()` is called.
        while iter.buffer.len() < window - 1 {
            iter.buffer.push_front(
                iter.source.next().expect("Iterator too short to fill BufMap buffer")
            );
        }
        iter
    }
}

// Implement `std::Iterator` on `BufMap`

impl<F, I> Iterator for BufMap<F, I>
    where F: Fn(&VecDeque<f32>) -> f32,
          I: Iterator<Item=f32> + Sized
{
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        match self.source.next() {
            Some(v) => {
                self.buffer.push_front(v);
                let result = (self.function)(&self.buffer);
                self.buffer.pop_back();
                Some(result)
            },
            None => None,
        }
    }
}


// Add method `buf_map()` to `std::iter::Iterator`

pub trait BufMapExt: Iterator {
    fn buf_map<F>(self, window: usize, function: F) -> BufMap<F, Self>
        where F: Fn(&VecDeque<f32>) -> f32,
              Self: Iterator<Item=f32> + Sized,
    {
        BufMap::new(self, window, function)
    }
}

impl<I: Iterator> BufMapExt for I {}




// SLOW ////////////////////////////////////////////////////////////////////////


/// Buffer accessible when using `slow_buf_map()`.
///
/// It's a wrapper around `VecDeque` that also owns a `Iterator`.
/// When calling `get()` this buffer takes a value from the `Iterator`.
///
/// The front of the `VecDeque` has the newest value. The back has the oldest
/// value.
pub struct SlowBuffer
{
    source: Box<Iterator<Item=f32>>,
    pub deque: VecDeque<f32>,
}

impl SlowBuffer
{
    pub fn new(iter: impl Iterator<Item=f32> + 'static) -> SlowBuffer {
        SlowBuffer {
            source: Box::new(iter),
            deque: VecDeque::new(),
        }
    }

    /// Fills the buffer to reach given length. Returns if succesful.
    pub fn fill(&mut self, length: usize) -> bool {
        let mut success = true;
        while self.deque.len() < length {
            success = self.pull()
        }
        return success;
    }

    /// Pull a value from the iterator. Returns if able to get value.
    pub fn pull(&mut self) -> bool {
        match self.source.next() {
            Some(v) => {
                self.deque.push_front(v);
                true
            },
            None => false,
        }
    }

    /// Pop oldest value on buffer.
    pub fn pop(&mut self) -> Option<f32> {
        self.deque.pop_back()
    }
}

// Implement indexing on `SlowBuffer`

impl std::ops::Index<i32> for SlowBuffer {
    type Output = f32;

    fn index(&self, mut index: i32) -> &f32 {
        if index < 0 {
            index = self.deque.len() as i32 + index;
        }
        if index < 0 {
            panic!("Negative buffer index out of bounds");
        }
        self.deque.get(index as usize)
            .expect("Positive buffer index out of bounds")
    }
}


/// An iterator that maps the values of an `Iterator` using a `SlowBuffer`.
pub struct SlowBufMap<F>
    where F: Fn(&SlowBuffer) -> f32
{
    buffer: SlowBuffer,
    function: F,
}

impl<F> SlowBufMap<F>
    where F: Fn(&SlowBuffer) -> f32
{
    pub fn new(iter: impl Iterator<Item=f32> + 'static, window: usize, function: F) -> SlowBufMap<F>
    {
        let mut iter = SlowBufMap {
            buffer: SlowBuffer::new(iter),
            function,
        };
        // Because I pull before next()
        if ! iter.buffer.fill(window - 1) {
            panic!("Iterator too short to fill SlowBufMap SlowBuffer");
        }
        iter
    }
}

// Implement `std::Iterator` on `SlowBufMap`

impl<F> Iterator for SlowBufMap<F>
    where F: Fn(&SlowBuffer) -> f32
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


// Add method `slow_buf_map()` to `std::iter::Iterator`

pub trait SlowBufMapExt: Iterator {
    fn slow_buf_map<F>(self, window: usize, function: F) -> SlowBufMap<F>
        where F: Fn(&SlowBuffer) -> f32,
              Self: Iterator<Item=f32> + 'static + Sized,
    {
        SlowBufMap::new(self, window, function)
    }
}

impl<I: Iterator> SlowBufMapExt for I {}
