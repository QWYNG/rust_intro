trait Init<T> {
    fn init(t: T) -> Self;
}

trait As<T> {
    fn cast(self) -> T;
}

impl<T> Init<T> for Box<T> {
    fn init(t: T) -> Self {
        Box::new(t)
    }
}

impl As<u64> for u8 {
    fn cast(self) -> u64 {
        self as u64
    }
}

impl As<u32> for u8 {
    fn cast(self) -> u32 {
        self as u32
    }
}

fn main() {
    let one_u32: u32 = 1.cast();
    let one_u32: u64 = 1.cast();
    println!("{}", one_u32)
}
