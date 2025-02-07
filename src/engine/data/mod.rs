
use cgmath::{Vector3, Zero};

use super::models::instance::instance_data::InstanceData;

#[derive(Debug)]
struct Aabb {
    min : Vector3<f32>, 
    max : Vector3<f32>, 
}

impl Aabb {
    pub fn contains(&self, pos: Vector3<f32>) -> bool {
        pos.x >= self.min.x && pos.x <= self.max.x &&
        pos.y >= self.min.y && pos.y <= self.max.y &&
        pos.z >= self.min.z && pos.z <= self.max.z
    }

    pub fn get_size(&self) -> Vector3<f32> {
        self.max - self.min
    }

    pub fn get_center(&self) -> Vector3<f32> {
        (self.max + self.min) / 2.0
    }

}

pub struct QuadtreeNode {
    bounds: Aabb, 
    children: Option<[Box<QuadtreeNode>; 8]>,
    is_full : bool, 
    remaining_subdivisions : u32, 
}

fn calc_subdivisions(mut size : f32) -> u32 {
    let mut subdv : u32 = 0; 
    while size > 1.0 {
        size /= 2.0;
        subdv += 1 
    }

    subdv 
}

impl QuadtreeNode {
    pub fn new(cells_per_side : u32) -> QuadtreeNode {

        let size = cells_per_side as f32; 

        QuadtreeNode {
            bounds: Aabb{
                min : Vector3::zero(), 
                max : Vector3::new(size, size, size),  
            }, 
            children: None, 
            is_full: false, 
            remaining_subdivisions: calc_subdivisions(size), 
        }
    }

    fn from_bounds(bounds : Aabb) -> QuadtreeNode {
        let size = bounds.get_size().x; 

        QuadtreeNode {
            bounds, 
            children: None, 
            is_full: false, 
            remaining_subdivisions: calc_subdivisions(size), 
        }
    }

    pub fn insert_voxel(&mut self, pos: Vector3<f32>) {
        // Check if we are at the max LOD (smallest subdivision)
        if self.remaining_subdivisions == 0 {
            self.is_full = true; 
            return;
        }

        // If this node has no children, create them
        if self.children.is_none() {
            let children = [
                Box::new(QuadtreeNode::from_bounds(self.subdivide(0))),
                Box::new(QuadtreeNode::from_bounds(self.subdivide(1))),
                Box::new(QuadtreeNode::from_bounds(self.subdivide(2))),
                Box::new(QuadtreeNode::from_bounds(self.subdivide(3))),
                Box::new(QuadtreeNode::from_bounds(self.subdivide(4))),
                Box::new(QuadtreeNode::from_bounds(self.subdivide(5))),
                Box::new(QuadtreeNode::from_bounds(self.subdivide(6))),
                Box::new(QuadtreeNode::from_bounds(self.subdivide(7))),
            ];
            self.children = Some(children);
        }

        // Forward voxel insertion to the correct child
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if child.bounds.contains(pos) {
                    child.insert_voxel(pos);

                    if children.iter().all(|child| child.is_full) {
                        self.is_full = true;
                        self.children = None; 
                    }

                    return;
                }
            }
        }
    }

    fn subdivide(&self, index: usize) -> Aabb {
        let center = self.bounds.get_center();

        match index {
            0 => Aabb { // Front-top-left
                min: self.bounds.min,
                max: Vector3::new(center.x, center.y, center.z),
            },
            1 => Aabb { // Front-top-right
                min: Vector3::new(center.x, self.bounds.min.y, self.bounds.min.z),
                max: Vector3::new(self.bounds.max.x, center.y, center.z),
            },
            2 => Aabb { // Front-bottom-left
                min: Vector3::new(self.bounds.min.x, center.y, self.bounds.min.z),
                max: Vector3::new(center.x, self.bounds.max.y, center.z),
            },
            3 => Aabb { // Front-bottom-right
                min: Vector3::new(center.x, center.y, self.bounds.min.z),
                max: Vector3::new(self.bounds.max.x, self.bounds.max.y, center.z),
            },
            4 => Aabb { // Back-top-left
                min: Vector3::new(self.bounds.min.x, self.bounds.min.y, center.z),
                max: Vector3::new(center.x, center.y, self.bounds.max.z),
            },
            5 => Aabb { // Back-top-right
                min: Vector3::new(center.x, self.bounds.min.y, center.z),
                max: Vector3::new(self.bounds.max.x, center.y, self.bounds.max.z),
            },
            6 => Aabb { // Back-bottom-left
                min: Vector3::new(self.bounds.min.x, center.y, center.z),
                max: Vector3::new(center.x, self.bounds.max.y, self.bounds.max.z),
            },
            7 => Aabb { // Back-bottom-right
                min: Vector3::new(center.x, center.y, center.z),
                max: self.bounds.max,
            },
            _ => unreachable!(),
        }
    }

    pub fn get_data(&self) -> Vec<InstanceData> {
        let mut positions = Vec::<InstanceData>::new();
        self.collect_leaf_positions(&mut positions);
        positions
    }

    fn collect_leaf_positions(&self, positions: &mut Vec<InstanceData>) {
        if let Some(children) = &self.children {
            for child in children.iter() {
                child.collect_leaf_positions(positions);
            }
        } else if self.is_full {
            let center = self.bounds.get_center(); 
            let size : Vector3<f32> = self.bounds.get_size();  
            let p = InstanceData {
                position: center.extend(1.0).into(), 
                size: size.extend(1.0).into(), 
            }; 
            positions.push(p);
        }
    }

}