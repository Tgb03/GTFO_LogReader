use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{dll_exports::callback_handler::HasCallbackHandler, seed_gen::consumers::{base_consumer::Consumer, key_id_consumer::KeyIDConsumer, ConsumerEnum}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct R4A2Wrapper {

    choices: VecDeque<Vec<KeyIDConsumer>>,
    consumers: VecDeque<ConsumerEnum>,

    #[serde(default, skip_serializing)]
    choice_done: VecDeque<KeyIDConsumer>,

}

impl<O> Consumer<O> for R4A2Wrapper
where
    O: HasCallbackHandler,
{
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        if let Some(mut val) = self.choices.pop_front() {
            self.choice_done.push_back(
                val.swap_remove((seed * val.len() as f32) as usize)
            );

            println!("eaten one choice: {seed}");

            return false;
        }
        
        if !self.consumers.is_empty() {
            self.consumers.take(seed, output);
        
            return false;
        }

        if let Some(mut choice) = self.choice_done.pop_front() {
            choice.take(seed, output);

            println!("cleaning up choice: {seed}");
        }

        self.choices.is_empty() && self.consumers.is_empty() && self.choice_done.is_empty()
    }
}
