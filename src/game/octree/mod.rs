use crate::engine::{engine::EngineData, models::instanced_model::InstancedModel};

mod octree_node;

pub struct Octree {
    pub root : octree_node::OctreeNode,
}

impl Octree {
    pub fn new() -> Self {
        Octree {
            root : octree_node::OctreeNode::new(0, 0, 0, 64, 0)
        }
    }

    pub fn insert(&mut self, x : u32, y : u32, z : u32, material_index : u32) {
        self.root.insert(x, y, z, material_index);
    }

    pub fn remove(&mut self, x : u32, y : u32, z : u32) {
        self.root.remove(x, y, z);
    }

    pub fn get(&self, x : u32, y : u32, z : u32) -> u32 {
        self.root.get(x, y, z)
    }

    pub fn as_model(&self, engine: &EngineData) -> InstancedModel {
        let instances = self.root.get_instances();
        InstancedModel::new(&engine.get_device(), "assets/cube.obj", instances)
    }
}