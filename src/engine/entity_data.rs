use super::models::{instanced_model::InstancedModel, standard_model::StandardModel};


pub struct EntityData<'a> {
    pub instanced_models : Vec<&'a InstancedModel>,
    pub models : Vec<&'a StandardModel>,
}

impl<'a> EntityData<'a> {
    pub fn new(instanced_models : Vec<&'a InstancedModel>, models : Vec<&'a StandardModel>) -> Self {
        EntityData {
            instanced_models,
            models,
        }
    }
}