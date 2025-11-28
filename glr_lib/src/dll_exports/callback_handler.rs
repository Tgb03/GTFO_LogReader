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

pub trait CallbackClone: Sized {
    fn clone_callbacks(self) -> Self;
}

impl<T> CallbackClone for T
where
    T: Sized + HasCallbackHandler + Default,
{
    fn clone_callbacks(self) -> Self {
        let mut s = Self::default();

        for (_, callback) in self.get_callback_handler() {
            s.add_callback(callback.clone());
        }

        s
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
                let context = callback.get_context();
                if let Some(result_string) = converter.convert(&data) {
                    event(context.get_ptr(), result_string.as_ptr())
                }
            }
        }
    }
}
