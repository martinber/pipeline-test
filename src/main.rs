extern crate hound;

fn main() {

    let a = vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.].into_iter();

    let b = duplicar(a);
    let c = duplicar(b);


    println!("{:?}", c.collect::<Vec<f32>>());


}

fn duplicar(entrada: impl std::iter::Iterator<Item=f32>) -> impl std::iter::Iterator<Item=f32> {

    entrada.map(|x| x * 2.)
}
/*
struct MyIter {
    buffer: Vec<f32>,
}

impl MyIter {


}

impl std::itrer::Iterator for MyIter {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        buffer.pop()
    }
}
*/
