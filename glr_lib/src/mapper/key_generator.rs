use glr_core::{data::KeyDescriptor, location::Location, token::Token};

use crate::mapper::location_generator::LocationGenerator;

/// generates ColoredKey & BulkheadKey
#[derive(Default)]
pub struct KeyGenerator {
    first_iteration: Option<KeyDescriptor>,
}

impl LocationGenerator for KeyGenerator {
    fn accept_token(&mut self, token: &Token) -> Option<Location> {
        match token {
            Token::ItemAllocated(key_descriptor) => {
                self.first_iteration = Some(key_descriptor.clone());

                None
            }
            Token::ItemSpawn(zone, id) => match self.first_iteration.take() {
                Some(key_descriptor) => Some(KeyDescriptor::into_location(
                    &key_descriptor,
                    *zone,
                    *id as u64,
                )),
                None => None,
            },
            _ => None,
        }
    }
}
