
use super::voxel_face_model::VoxelFaceModel;

pub trait DrawModel<'a> {
    fn draw_voxel_instanced(&mut self,bind_group_index: u32,model: &'a VoxelFaceModel);
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
    where 'b: 'a, 
{
    fn draw_voxel_instanced(
        &mut self,
        _bind_group_index: u32,
        model: &'a VoxelFaceModel
    ) {
        self.set_vertex_buffer(0, model.mesh.vertex_data.slice(..));
        self.set_index_buffer(model.mesh.index_data.slice(..), wgpu::IndexFormat::Uint32);
        self.set_vertex_buffer(1, model.instance_buffer.slice(..));
        self.draw_indexed(0..model.mesh.num_elements, 0, 0..model.instance_count);
    }
}
