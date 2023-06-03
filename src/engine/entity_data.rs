use super::{env::light::LightData, models::{instanced_model::InstancedModel, standard_model::StandardModel, indirect_model::IndirectModel}};


pub struct EntityData<'a> {
    pub lights : Vec<LightData>,
    pub instanced_models : Vec<&'a InstancedModel>,
    pub indirect_models : Vec<&'a IndirectModel>,
    pub models : Vec<&'a StandardModel>,
}

impl<'a> EntityData<'a> {
    pub fn new(lights : Vec<LightData>, instanced_models : Vec<&'a InstancedModel>, indirect_models : Vec<&'a IndirectModel>, models : Vec<&'a StandardModel>) -> Self {
        EntityData {
            lights,
            instanced_models,
            indirect_models,
            models,
        }
    }
}