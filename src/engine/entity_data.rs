use super::{env::light::LightData, models::{instanced_model::InstancedModel, model::Model}};


pub struct EntityData<'a> {
    pub lights : Vec<LightData>,
    pub instanced_models : Vec<&'a InstancedModel>,
    pub models : Vec<&'a Model>,
}

impl<'a> EntityData<'a> {
    pub fn new(lights : Vec<LightData>, instanced_models : Vec<&'a InstancedModel>, models : Vec<&'a Model>) -> Self {
        EntityData {
            lights,
            instanced_models,
            models,
        }
    }

    #[allow(dead_code)]
    pub fn add_light(&mut self, light : LightData) {
        self.lights.push(light);
    }

    #[allow(dead_code)]
    pub fn add_instanced_model(&mut self, model : &'a InstancedModel) {
        self.instanced_models.push(model);
    }

    #[allow(dead_code)]
    pub fn add_model(&mut self, model : &'a Model) {
        self.models.push(model);
    }
}