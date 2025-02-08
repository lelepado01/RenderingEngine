
use bytemuck::{Pod, Zeroable};

use super::buffers::traits::AsStorageBuffer;
use super::buffers::storage_buffer::StorageBuffer; 

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct VoxelMaterial {
    pub diffuse_color: [f32; 4],   // RGBA
    pub specular_color: [f32; 3],  // RGB
    pub shininess: f32,            // Shininess factor
    pub metallic: f32,             // PBR metallic
    pub roughness: f32,            // PBR roughness
    pub _padding: [f32; 2],        // Align to 16 bytes (wgpu requirement)
}

impl VoxelMaterial {
    pub const fn green() -> VoxelMaterial {
        VoxelMaterial { 
            diffuse_color: [0.1, 0.8, 0.1, 1.0], // Green
            specular_color: [0.1, 0.8, 0.1], 
            shininess: 32.0,
            metallic: 0.0,
            roughness: 1.0,
            _padding: [1.0, 1.0], 
        }
    }

    pub const fn red() -> VoxelMaterial {
        VoxelMaterial { 
            diffuse_color: [0.8, 0.1, 0.1, 1.0], // Red
            specular_color: [0.8, 0.1, 0.1], 
            shininess: 32.0,
            metallic: 0.0,
            roughness: 1.0,
            _padding: [1.0, 1.0], 
        }
    }

    pub const fn blue() -> VoxelMaterial {
        VoxelMaterial { 
            diffuse_color: [0.1, 0.1, 0.8, 1.0], // Blue
            specular_color: [0.1, 0.1, 0.8], 
            shininess: 32.0,
            metallic: 0.0,
            roughness: 1.0,
            _padding: [1.0, 1.0], 
        }
    }

    pub const fn yellow() -> VoxelMaterial {
        VoxelMaterial { 
            diffuse_color: [0.9, 0.9, 0.1, 1.0], // Yellow
            specular_color: [0.9, 0.9, 0.1], 
            shininess: 32.0,
            metallic: 0.0,
            roughness: 1.0,
            _padding: [1.0, 1.0], 
        }
    }

    pub const fn cyan() -> VoxelMaterial {
        VoxelMaterial { 
            diffuse_color: [0.1, 0.9, 0.9, 1.0], // Cyan
            specular_color: [0.1, 0.9, 0.9], 
            shininess: 32.0,
            metallic: 0.0,
            roughness: 1.0,
            _padding: [1.0, 1.0], 
        }
    }

    pub const fn magenta() -> VoxelMaterial {
        VoxelMaterial { 
            diffuse_color: [0.9, 0.1, 0.9, 1.0], // Magenta
            specular_color: [0.9, 0.1, 0.9], 
            shininess: 32.0,
            metallic: 0.0,
            roughness: 1.0,
            _padding: [1.0, 1.0], 
        }
    }

    pub const fn white() -> VoxelMaterial {
        VoxelMaterial { 
            diffuse_color: [1.0, 1.0, 1.0, 1.0], // White
            specular_color: [1.0, 1.0, 1.0], 
            shininess: 64.0,
            metallic: 0.0,
            roughness: 0.5,
            _padding: [1.0, 1.0], 
        }
    }

    pub const fn black() -> VoxelMaterial {
        VoxelMaterial { 
            diffuse_color: [0.0, 0.0, 0.0, 1.0], // Black
            specular_color: [0.0, 0.0, 0.0], 
            shininess: 8.0,
            metallic: 0.0,
            roughness: 1.0,
            _padding: [1.0, 1.0], 
        }
    }
}

pub const MATERIAL_PALETTE : [VoxelMaterial; 8] = [
    VoxelMaterial::black(), 
    VoxelMaterial::blue(), 
    VoxelMaterial::cyan(), 
    VoxelMaterial::green(), 
    VoxelMaterial::magenta(), 
    VoxelMaterial::red(), 
    VoxelMaterial::white(), 
    VoxelMaterial::yellow()
]; 


fn fill_material_data(material_data : &mut Vec::<f32>, vm : &VoxelMaterial) {
    material_data.extend(vm.diffuse_color);
    material_data.extend(vm.specular_color);
    material_data.extend([vm.shininess, vm.metallic, vm.roughness]);
    material_data.extend(vm._padding);
}

impl AsStorageBuffer for [VoxelMaterial; 8] {
    fn as_storage_buffer(&self, device : &wgpu::Device) -> StorageBuffer {
        let mut material_data = Vec::<f32>::new();
        
        self.iter().for_each(|m| fill_material_data(&mut material_data, m));

        let buffer_size = std::mem::size_of::<f32>() * material_data.len();
        StorageBuffer::new(device, &material_data, buffer_size as u64)
    }
}