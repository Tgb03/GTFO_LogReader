use std::sync::mpsc::Sender;

pub trait OutputTrait<D> {
    fn output(&mut self, data: D);
}

impl<D> OutputTrait<D> for Vec<D> {
    fn output(&mut self, data: D) {
        self.push(data);
    }
}

impl<D> OutputTrait<D> for Sender<D> {
    fn output(&mut self, data: D) {
        let _ = self.send(data);
    }
}