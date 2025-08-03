use std::fmt::Debug;

pub trait OutputTrait<D> {
    fn output(&self, data: D);
}

#[derive(Default)]
pub struct PrintOutput;

impl<T: Debug> OutputTrait<T> for PrintOutput {
    fn output(&self, data: T) {
        println!("{:?}", data);
    }
}
