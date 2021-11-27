use super::*;

enum Direction {
    Vertical,
    Horizont,
}

pub struct Flex<T: Data> {
    sub: Vec<Box<dyn Widget<T>>>,
}
