pub trait OutputTrait<D> {
    fn output(&mut self, data: D);
}

impl<D> OutputTrait<D> for Vec<D> {
    fn output(&mut self, data: D) {
        self.push(data);
    }
}