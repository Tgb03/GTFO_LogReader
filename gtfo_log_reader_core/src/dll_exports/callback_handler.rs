use std::collections::HashMap;

use serde::Serialize;

use crate::{dll_exports::structs::CallbackInfo, output_trait::OutputTrait};

pub trait HasCallbackHandler {
    fn get_callback_handler(&self) -> &HashMap<u32, CallbackInfo>;
    fn get_callback_handler_mut(&mut self) -> &mut HashMap<u32, CallbackInfo>;

    fn add_callback(&mut self, callback: CallbackInfo) {
        self.get_callback_handler_mut()
            .insert(callback.get_id(), callback);
    }

    fn remove_callback(&mut self, callback_id: u32) {
        self.get_callback_handler_mut().remove(&callback_id);
    }
}

impl<O, H> OutputTrait<O> for H
where
    O: Serialize,
    H: HasCallbackHandler,
{
    fn output(&self, data: O) {
        for callback in self.get_callback_handler().values() {
            if let Some(event) = callback.get_event_callback() {
                let converter = callback.get_message_type();
                if let Some(result_string) = converter.convert(&data) {
                    event(result_string.as_ptr())
                }
            }
        }
    }
}
