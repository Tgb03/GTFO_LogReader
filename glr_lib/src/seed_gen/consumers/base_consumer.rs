use enum_dispatch::enum_dispatch;
use glr_core::seed_indexer_result::OutputSeedIndexer;

use crate::{
    output_trait::OutputTrait,
};
use std::{collections::VecDeque, fmt::Debug};

#[enum_dispatch]
pub trait Consumer<O>: Debug
where
    O: OutputTrait<OutputSeedIndexer>,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O);
}

impl<O> Consumer<O> for VecDeque<Box<dyn Consumer<O>>>
where
    O: OutputTrait<OutputSeedIndexer>,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        for it in self {
            it.take(seed_iter, output);
        }
    }
}

impl<O, T> Consumer<O> for Vec<T>
where
    O: OutputTrait<OutputSeedIndexer>,
    T: Debug + Consumer<O>,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        for it in self {
            it.take(seed_iter, output);
        }
    }
}

impl<O, T> Consumer<O> for &Vec<T>
where
    O: OutputTrait<OutputSeedIndexer>,
    T: Debug + Consumer<O>,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        for it in self.iter() {
            it.take(seed_iter, output);
        }
    }
}
