use crate::engine::utils::array_math::{Add, ScalarDiv};


pub mod instanced_vertex; 
pub mod instance_data;
pub mod standard_vertex;

pub enum VertexType {
    InstancedVertex,
    StandardVertex,
}

pub trait VertexData {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

pub trait Parsable {
    fn from_mesh(index : usize, mesh : &tobj::Mesh) -> Self;
}

pub trait CalculateNormals {
    fn calculate_normals(&mut self, indices : &Vec<u32>);
}

impl CalculateNormals for Vec<standard_vertex::StandardModelVertex> {
    fn calculate_normals(&mut self, indices : &Vec<u32>) {
        for i in (0..indices.len()-3).step_by(3) {
            let v1 = self[indices[i] as usize]._pos;
            let v2 = self[indices[i + 1] as usize]._pos;
            let v3 = self[indices[i + 2] as usize]._pos;

            let normal = calculate_face_normal(v1, v2, v3);

            self[indices[i] as usize]._normal.add(normal);
            self[indices[i + 1] as usize]._normal.add(normal);
            self[indices[i + 2] as usize]._normal.add(normal);
        }

        for i in 0..self.len() {
            self[i]._normal = self[i]._normal.scalar_div(3.0);
        }
    }
}

impl CalculateNormals for Vec<instanced_vertex::InstancedModelVertex> {
    fn calculate_normals(&mut self, indices : &Vec<u32>) {
        for i in (0..indices.len()-3).step_by(3) {
            let v1 = self[indices[i] as usize]._pos;
            let v2 = self[indices[i + 1] as usize]._pos;
            let v3 = self[indices[i + 2] as usize]._pos;

            let normal = calculate_face_normal(v1, v2, v3);

            self[indices[i] as usize]._normal.add(normal);
            self[indices[i + 1] as usize]._normal.add(normal);
            self[indices[i + 2] as usize]._normal.add(normal);
        }

        for i in 0..self.len() {
            self[i]._normal = self[i]._normal.scalar_div(3.0);
        }
    }
}

pub fn calculate_face_normal(v1 : [f32; 4], v2 : [f32; 4], v3 : [f32; 4]) -> [f32; 4] {
    let mut normal = [0.0, 0.0, 0.0, 1.0];

    let mut v1v2 = [0.0, 0.0, 0.0];
    let mut v1v3 = [0.0, 0.0, 0.0];

    v1v2[0] = v2[0] - v1[0];
    v1v2[1] = v2[1] - v1[1];
    v1v2[2] = v2[2] - v1[2];

    v1v3[0] = v3[0] - v1[0];
    v1v3[1] = v3[1] - v1[1];
    v1v3[2] = v3[2] - v1[2];

    normal[0] = v1v2[1] * v1v3[2] - v1v2[2] * v1v3[1];
    normal[1] = v1v2[2] * v1v3[0] - v1v2[0] * v1v3[2];
    normal[2] = v1v2[0] * v1v3[1] - v1v2[1] * v1v3[0];

    let mut length = 0.0;
    length += normal[0] * normal[0];
    length += normal[1] * normal[1];
    length += normal[2] * normal[2];
    length = length.sqrt();

    normal[0] = normal[0] / length;
    normal[1] = normal[1] / length;
    normal[2] = normal[2] / length;

    return normal;
}
