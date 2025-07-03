use serde::{Deserialize, Serialize};

use crate::{
    dll_exports::callback_handler::HasCallbackHandler, seed_gen::consumers::base_consumer::Consumer,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct IgnoreConsumer {
    count: usize,
}

impl IgnoreConsumer {

    pub fn new(count: usize) -> Self {
        Self { count }
    }

}

impl<O> Consumer<O> for IgnoreConsumer
where
    O: HasCallbackHandler,
{
    fn take(&mut self, _: f32, _: &mut O) -> bool {
        self.count -= 1;

        self.count == 0
    }
}
