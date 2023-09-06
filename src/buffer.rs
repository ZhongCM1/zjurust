struct Buffer<T> {
    pub mem: Vec<T>,
}

impl<T> Buffer<T> 
    where
    T: std::ops::Add<T, Output = T> + Default + Clone,
{
    pub fn new() -> Self{
        Buffer{mem: Vec::new() }
    }
    pub fn sum(&self) -> T {
        let mut sum: T = T::default();
        for elm in &self.mem {
            sum = sum + elm.clone();
        }
        sum
    }
}

fn main() {
    let mut buffer: Buffer<i32> = Buffer::new();
    for num in 1..10 {
        buffer.mem.push(num);
    }
    println!("sum = {}", buffer.sum());
}