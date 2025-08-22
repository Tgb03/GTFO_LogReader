use serde::{Deserialize, Serialize};

use crate::{dll_exports::callback_handler::HasCallbackHandler, seed_gen::{consumers::{base_consumer::Consumer, ConsumerEnum}, zone_info::{collectable_alloc::CollectableAlloc, generated_data::GeneratedZone}}};


pub type Task<H> = Box<dyn Fn(&mut Vec<GeneratedZone>, &mut dyn Iterator<Item = f32>, &mut H) -> ()>;


pub fn make_empty_task<H>() -> Task<H> {
    Box::new(|_, _, _| {})
}


pub trait StagedObjective<O: HasCallbackHandler> {

    fn get_task(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) -> Task<O>;

}


impl<O> StagedObjective<O> for ConsumerEnum
where 
    O: HasCallbackHandler {
    
    fn get_task(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) -> Task<O> {
        self.take(seed_iter, output);
        
        make_empty_task()
    }

}


#[derive(Debug, Serialize, Deserialize)]
pub enum StagedObjectiveEnum {

    Consumer(ConsumerEnum),
    GatherObjective(CollectableAlloc),

}


impl<O> StagedObjective<O> for StagedObjectiveEnum
where 
    O: HasCallbackHandler {
    
    fn get_task(&self, seed_iter: &mut dyn Iterator<Item = f32>, output: &mut O) -> Task<O> {
        match self {
            StagedObjectiveEnum::Consumer(consumer_enum) => consumer_enum.get_task(seed_iter, output),
            StagedObjectiveEnum::GatherObjective(collectable_alloc) => collectable_alloc.get_task(seed_iter, output),
        }
    }

}

