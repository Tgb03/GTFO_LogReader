use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{dll_exports::callback_handler::HasCallbackHandler, seed_gen::consumers::{base_consumer::Consumer, key_id_consumer::KeyIDConsumer, ConsumerEnum}};

#[derive(Debug, Serialize, Deserialize)]
pub struct R4A2Wrapper {

    choices: VecDeque<Vec<KeyIDConsumer>>,
    consumers: VecDeque<ConsumerEnum>,

}

impl<O> Consumer<O> for R4A2Wrapper
where
    O: HasCallbackHandler,
{
    fn take(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) {
        let mut choice_done = Vec::new();
        for it in &self.choices {
            choice_done.push(
                it[(seed_iter.next().unwrap() * it.len() as f32) as usize].clone()
            );
        }
        
        self.consumers.take(seed_iter, output);

        choice_done.take(seed_iter, output);
    }
}
