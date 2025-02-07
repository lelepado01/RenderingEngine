pub mod uniform_buffer;
pub mod storage_buffer;
pub mod traits;

use wgpu::util::DeviceExt; 


#[derive(Debug, Clone, Copy)]
pub enum BufferType {
    Vertex,
    Index,
    Uniform,
    Storage,
    Instance,
}

pub fn create_buffer<T>(
    device: &wgpu::Device,
    buffer_type: BufferType,
    data: &[T],
) -> wgpu::Buffer
where T : bytemuck::Pod + bytemuck::Zeroable
{
    let usage = match buffer_type {
        BufferType::Vertex => wgpu::BufferUsages::VERTEX,
        BufferType::Index => wgpu::BufferUsages::INDEX,
        BufferType::Uniform => wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        BufferType::Instance => wgpu::BufferUsages::VERTEX,
        BufferType::Storage => wgpu::BufferUsages::STORAGE,
    };
    
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(data),
        usage,
    })
}