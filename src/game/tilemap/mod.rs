use crate::engine::{models::{vertices::instance_data::PositionInstanceData, instanced_model::{InstancedModel, self}}, engine::EngineData, camera::{self, Camera}};
mod chunk;
use chunk::TileChunk;

pub struct Tile {
    pub position: [f32; 3],
    pub material: f32,
}

pub struct TileMap {
    pub tiles: Vec<Tile>,
    pub chunks: Vec<TileChunk>,
    max_distance: f32,
    chunk_size : f32,
}

impl TileMap {
    pub fn new() -> Self {

        let mut tilemap= TileMap {
            tiles: Vec::new(),
            chunks: Vec::new(),
            max_distance: 10000.0,
            chunk_size: 30.0,
        };
        
        for i in 0..10 {
            for j in 0..10 {
                let chunk = TileChunk::new(i, j, tilemap.chunk_size, [i as f32 * 2.0 * tilemap.chunk_size, 0.0, j as f32 * 2.0 * tilemap.chunk_size]);
                tilemap.chunks.push(chunk);
            }
        }

        tilemap

    }

    pub fn update<T>(&mut self, delta_time: f32, camera : &T, engine: &EngineData) 
        where T: Camera
    {

        let cam_pos = camera.get_position();

        // remove chunks that are too far away
        self.chunks.retain(|x| {
            let dist = (x.center[0] - cam_pos[0]).powf(2.0) + (x.center[2] - cam_pos[2]).powf(2.0);
            dist < self.max_distance
        });

        let cam_chunk_pos = [(cam_pos[0] / (2.0 * self.chunk_size)) as i32, (cam_pos[2] / (2.0 * self.chunk_size)) as i32];

        for i in cam_chunk_pos[0]..(cam_chunk_pos[0]+10) {
            for j in cam_chunk_pos[1]..(cam_chunk_pos[1]+10) {
                let chunk_pos = [i as f32 * 2.0 * self.chunk_size, 0.0, j as f32 * 2.0 * self.chunk_size];

                if self.chunks.iter().any(|x| x.chunk_coords == [i, j]) {
                    continue;
                }

                let dist = (chunk_pos[0] - cam_pos[0]).powf(2.0) + (chunk_pos[2] - cam_pos[2]).powf(2.0);
                if dist < self.max_distance {
                    let chunk = TileChunk::new(i, j, self.chunk_size, chunk_pos);
                    self.chunks.push(chunk);
                }
            }
        }

    }

    pub fn as_model(&self, engine: &EngineData) -> InstancedModel {

        let instances : Vec<PositionInstanceData> = self.chunks
            .iter()
            .map(|x| 
                x.tiles
                    .iter()
                    .map(|x| PositionInstanceData { position: [x.position[0], x.position[1], x.position[2], 1.0], material_index: [x.material, 0.0, 0.0, 0.0] })
                    .collect::<Vec<PositionInstanceData>>()
            )
            .flatten()
            .collect(); 

        let model = instanced_model::InstancedModel::new(
            &engine.get_device(), 
            "assets/cube.obj", 
            instances,
        ); 

        model
    }
}