
use crate::engine::buffers::{storage_buffer, material_buffer::{MaterialBuffer, SetMaterialBuffer}};

use super::{mesh::Mesh, model::Model, instanced_model::InstancedModel};

pub trait DrawModel<'a> {
    fn draw_mesh(&mut self, 
        bind_group_index: u32, 
        mesh: &'a Mesh, 
        material_buffer: &'a MaterialBuffer
    );

    fn draw_model(&mut self, bind_group_index: u32, model: &'a Model);
    fn draw_model_instanced(
        &mut self,
        bind_group_index: u32,
        model: &'a InstancedModel,
    ); 
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{

    fn draw_mesh(&mut self, bind_group_index: u32, mesh: &'b Mesh, material_buffer: &'a MaterialBuffer) {
        self.set_vertex_buffer(0, mesh.vertex_data.slice(..));
        self.set_index_buffer(mesh.index_data.slice(..), wgpu::IndexFormat::Uint32);
        // self.set_bind_group(bind_group_index, &material_buffer.bind_group, &[]);
        self.set_material_buffer(bind_group_index, mesh.material_index, material_buffer);
        self.draw_indexed(0..mesh.num_elements, 0, 0..1);
    }

    fn draw_model(&mut self, bind_group_index: u32, model: &'b Model) {
        for mesh in &model.meshes {
            // let material = &model.materials[mesh.material_index];
            self.draw_mesh(bind_group_index, mesh, &model.material_buffer.as_ref().unwrap());
        }
    }

    fn draw_model_instanced(
            &mut self,
            bind_group_index: u32,
            model: &'a InstancedModel,
    ) {
        for mesh in &model.meshes {
            // let material = &model.materials[mesh.material_index];
            self.set_vertex_buffer(0, mesh.vertex_data.slice(..));
            self.set_index_buffer(mesh.index_data.slice(..), wgpu::IndexFormat::Uint32);
            self.set_bind_group(bind_group_index, &model.material_buffer.bind_group, &[]);
            self.set_vertex_buffer(1, model.instance_buffer.slice(..));
            self.draw_indexed(0..mesh.num_elements, 0, 0..model.instance_count);
        }
    }
}
