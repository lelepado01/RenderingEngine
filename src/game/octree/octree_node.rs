use crate::engine::models::vertices::instance_data::PositionInstanceData;

pub struct OctreeNode {
    pub x : u32,
    pub y : u32,
    pub z : u32,
    pub size : u32,
    pub material_index : u32,
    pub children : Option<[Box<OctreeNode>; 8]>,
}

impl OctreeNode {
    pub fn new(x : u32, y : u32, z : u32, size : u32, material_index : u32) -> Self {
        OctreeNode {
            x, y, z,
            size,
            material_index,
            children : None,
        }
    }

    pub fn join(&mut self) {
        if let Some(children) = &mut self.children {
            let mut all_same: bool = true;
            let first_material_index = children[0].material_index;
            for child in children.iter() {
                if child.material_index != first_material_index || !child.is_leaf() {
                    all_same = false;
                    break;
                }
            }
            if all_same {
                self.material_index = first_material_index;
                self.children = None;
            }
        }
    }

    pub fn insert(&mut self, x : u32, y : u32, z : u32, material_index : u32) {
        if self.size == 1 {
            self.material_index = material_index;
            return;
        }

        let half_size = self.size / 2;
        let mut child_index = 0;
        if x >= self.x + half_size {
            child_index += 1;
        }
        if y >= self.y + half_size {
            child_index += 2;
        }
        if z >= self.z + half_size {
            child_index += 4;
        }

        if self.children.is_none() {
            self.children = Some([
                Box::new(OctreeNode::new(self.x, self.y, self.z, half_size, 0)),
                Box::new(OctreeNode::new(self.x + half_size, self.y, self.z, half_size, 0)),
                Box::new(OctreeNode::new(self.x, self.y + half_size, self.z, half_size, 0)),
                Box::new(OctreeNode::new(self.x + half_size, self.y + half_size, self.z, half_size, 0)),
                Box::new(OctreeNode::new(self.x, self.y, self.z + half_size, half_size, 0)),
                Box::new(OctreeNode::new(self.x + half_size, self.y, self.z + half_size, half_size, 0)),
                Box::new(OctreeNode::new(self.x, self.y + half_size, self.z + half_size, half_size, 0)),
                Box::new(OctreeNode::new(self.x + half_size, self.y + half_size, self.z + half_size, half_size, 0)),
            ]);
        }

        self.children.as_mut().unwrap()[child_index].insert(x, y, z, material_index);

        self.join();
    }

    pub fn remove(&mut self, x : u32, y : u32, z : u32) {
        if self.size == 1 {
            self.material_index = 0;
            return;
        }

        let half_size = self.size / 2;
        let mut child_index = 0;
        if x >= self.x + half_size {
            child_index += 1;
        }
        if y >= self.y + half_size {
            child_index += 2;
        }
        if z >= self.z + half_size {
            child_index += 4;
        }

        if self.children.is_some() {
            self.children.as_mut().unwrap()[child_index].remove(x, y, z);
        }

        self.join();
    }

    pub fn get(&self, x : u32, y : u32, z : u32) -> u32 {
        if self.size == 1 {
            return self.material_index;
        }

        let half_size = self.size / 2;
        let mut child_index = 0;
        if x >= self.x + half_size {
            child_index += 1;
        }
        if y >= self.y + half_size {
            child_index += 2;
        }
        if z >= self.z + half_size {
            child_index += 4;
        }

        self.children.as_ref().unwrap()[child_index].get(x, y, z)
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    pub fn get_instances(&self) -> Vec<PositionInstanceData> {
        let mut instances = Vec::new();

        if self.is_leaf() {
            if self.material_index != 0 {
                instances.push(PositionInstanceData {
                    position : [self.x as f32, self.y as f32, self.z as f32, self.size as f32],
                    material_index : [self.material_index as f32, 0.0, 0.0, 0.0],
                });
                return instances;
            }
        } else {
            for child in self.children.as_ref().unwrap() {
                instances.append(&mut child.get_instances());
            }
        }   

        instances
    }
}
