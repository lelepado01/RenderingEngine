
use crate::engine::buffers::material_buffer::{MaterialBuffer, SetMaterialBuffer};

use super::{mesh::Mesh, standard_model::StandardModel, instanced_model::InstancedModel, indirect_model::IndirectModel};

pub trait DrawModel<'a> {
    fn draw_mesh(&mut self, bind_group_index: u32, mesh: &'a Mesh, material_buffer: &'a MaterialBuffer);
    fn draw_model(&mut self, bind_group_index: u32, model: &'a StandardModel);
    fn draw_model_instanced(&mut self,bind_group_index: u32,model: &'a InstancedModel); 
    fn draw_model_indirect(&mut self,bind_group_index: u32,model: &'a IndirectModel);
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
    where 'b: 'a, 
{

    fn draw_mesh(&mut self, bind_group_index: u32, mesh: &'b Mesh, material_buffer: &'a MaterialBuffer) {
        self.set_vertex_buffer(0, mesh.vertex_data.slice(..));
        self.set_index_buffer(mesh.index_data.slice(..), wgpu::IndexFormat::Uint32);
        self.set_material_buffer(bind_group_index, mesh.material_index, material_buffer);
        self.draw_indexed(0..mesh.num_elements, 0, 0..1);
    }

    fn draw_model(&mut self, bind_group_index: u32, model: &'b StandardModel) {
        for mesh in &model.meshes {
            self.draw_mesh(bind_group_index, mesh, &model.material_buffer);
        }
    }

    fn draw_model_instanced(
            &mut self,
            bind_group_index: u32,
            model: &'a InstancedModel,
    ) {
        for mesh in &model.meshes {
            self.set_vertex_buffer(0, mesh.vertex_data.slice(..));
            self.set_index_buffer(mesh.index_data.slice(..), wgpu::IndexFormat::Uint32);
            self.set_bind_group(bind_group_index, &model.material_buffer.bind_group, &[]);
            self.set_vertex_buffer(1, model.instance_buffer.slice(..));
            self.draw_indexed(0..mesh.num_elements, 0, 0..model.instance_count);
        }
    }

    fn draw_model_indirect(
            &mut self,
            bind_group_index: u32,
            model: &'a IndirectModel,
    ) {
        for mesh in &model.meshes {
            self.set_vertex_buffer(0, mesh.vertex_data.slice(..));
            self.set_index_buffer(mesh.index_data.slice(..), wgpu::IndexFormat::Uint32);
            self.set_vertex_buffer(1, model.instance_buffer.slice(..)); 
            // self.set_bind_group(bind_group_index, &model.material_buffer.bind_group, &[]);
            self.draw_indexed_indirect(&model.indirect_buffer, 0);
        }
    }
}
