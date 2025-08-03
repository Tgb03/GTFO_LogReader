use enum_dispatch::enum_dispatch;
use glr_core::seed_indexer_result::OutputSeedIndexer;

use crate::{
    dll_exports::callback_handler::HasCallbackHandler,
    output_trait::OutputTrait,
    seed_gen::consumers::ConsumerEnum,
};
use std::{collections::VecDeque, fmt::Debug};

#[enum_dispatch]
pub trait Consumer<O>: Debug
where
    O: HasCallbackHandler,
{
    /// take one seed and return whether or not
    /// the consumer is done eating seeds
    ///
    /// true -> consumer is done
    /// false -> consumer can still eat seeds
    fn take(&mut self, seed: f32, output: &mut O) -> bool;

    /// takes seeds until the consumer is completely
    /// finished. If the number of seeds is not enough,
    /// behaviour is to just stop even if unfinished.
    fn take_multiple(&mut self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) -> bool {
        while let Some(seed) = seed_iter.next() {
            if self.take(seed, output) {
                return true;
            }
        }

        false
    }
}

impl<O> Consumer<O> for VecDeque<Box<dyn Consumer<O>>>
where
    O: HasCallbackHandler + OutputTrait<OutputSeedIndexer>,
{
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        if let Some(consumer) = self.get_mut(0) {
            if consumer.take(seed, output) {
                self.pop_front();
            }
        }

        self.is_empty()
    }
}

impl<O> Consumer<O> for VecDeque<ConsumerEnum>
where
    O: HasCallbackHandler + OutputTrait<OutputSeedIndexer>,
{
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        if let Some(consumer) = self.get_mut(0) {
            if consumer.take(seed, output) {
                self.pop_front();
            }
        }

        self.is_empty()
    }
}
