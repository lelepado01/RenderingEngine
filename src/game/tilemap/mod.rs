use bytemuck::{Zeroable, Pod};

use crate::engine::{engine::EngineData, models::{instanced_model::{InstancedModel, self}, vertices::VertexData}};


#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct MaterialInstanceData {
    material_index: u32,
}

impl VertexData for MaterialInstanceData {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<MaterialInstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

pub struct PositionalTileMap {
    xsize : u32,
    ysize : u32,
    zsize : u32,

    instances : Vec<MaterialInstanceData>,
    model : Option<InstancedModel>,
}


impl PositionalTileMap {
    pub fn new() -> Self {
        let mut m = PositionalTileMap {
            xsize : 250,
            ysize : 50,
            zsize : 250,
            instances : vec![],

            model : None,
        }; 

        m.instances = vec![MaterialInstanceData{material_index: 0}; (m.xsize * m.ysize * m.zsize) as usize];

        for i in 0..m.xsize {
            for j in 0..m.ysize {
                for k in 0..m.zsize {
                    let index = m.xyz_to_index(i, j, k);
                    let material_index = (i) % 2;
                    m.instances[index as usize] = MaterialInstanceData{material_index: material_index};
                }
            }
        }

        m
    }

    pub fn xyz_to_index(&self, x : u32, y : u32, z : u32) -> u32 {
        z * self.xsize * self.ysize + y * self.xsize + x
    }

    #[allow(dead_code)]
    pub fn index_to_xyz(&self, index : u32) -> (u32, u32, u32) {
        let x = index / (self.xsize * self.ysize);
        let y = (index % (self.xsize * self.ysize)) / self.zsize;
        let z = index % self.zsize;

        (x, y, z)
    }

    pub fn as_model(&mut self, engine: &EngineData) -> &InstancedModel {

        if self.model.is_some() {
            self.model.as_mut().unwrap().update_instances(&engine.get_device(), &self.instances);
        } else {
            self.model = Some(instanced_model::InstancedModel::new(
                &engine.get_device(), 
                "assets/cube.obj", 
                self.instances.clone(),
            )); 
        }

        self.model.as_ref().unwrap()
    }
}