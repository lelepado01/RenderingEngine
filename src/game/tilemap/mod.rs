mod chunk;
mod tile;
mod aesthetics;

use crate::engine::{models::{vertices::instance_data::PositionInstanceData, instanced_model::{InstancedModel, self}}, engine::EngineData, utils::array_math::ScalarMul};
use crate::physics::get_distance; 

const MAP_SIZE_X : usize = 5400;
const MAP_SIZE_Y : usize = 2700;
const MAP_HEIGHT : f32 = 250.0;

pub struct TileMap {
    pub chunks: Vec<chunk::TileChunk>,
    max_distance: f32,
    chunks_in_view : i32,
    chunk_size : f32,
    tile_size : f32,
    model: std::option::Option<InstancedModel>,

    aesthetics: aesthetics::MapAestheticsParams,
}

impl TileMap {
    pub fn new() -> Self {
        let mut tilemap= TileMap {
            chunks: Vec::new(),
            max_distance: 0.0,
            chunks_in_view: 5,
            chunk_size: 30.0,
            tile_size: 2.0,
            model: None,
            aesthetics: aesthetics::MapAestheticsParams::new(),
        };

        tilemap.max_distance = tilemap.chunks_in_view as f32 * tilemap.chunk_size * tilemap.tile_size;
        
        for i in tilemap.get_chunks_in_view(0) {
            for j in tilemap.get_chunks_in_view(0) {
                let chunk = chunk::TileChunk::new(i, j, &mut tilemap);
                tilemap.chunks.push(chunk);
            }
        }

        tilemap
    }

    pub fn update(&mut self, player_position : &[f32; 3]) {

        let scaled_pp = player_position.scalar_mul(self.tile_size); 

        self.chunks.retain(|x| { 
            get_distance(&x.center, &scaled_pp) < self.max_distance 
        });

        let cam_chunk_pos = self.to_chunk_coords(scaled_pp); 

        for i in self.get_chunks_in_view(cam_chunk_pos[0]) {
            for j in self.get_chunks_in_view(cam_chunk_pos[1]) {
                
                if self.chunks.iter().any(|x| x.chunk_coords == [i, j]) {
                    continue;
                }
                
                let chunk_pos = self.to_world_coords([i, j]); 
                let dist = get_distance(&chunk_pos, &scaled_pp);
                if dist < self.max_distance {
                    let chunk = chunk::TileChunk::new(i, j, self);
                    self.chunks.push(chunk);
                }
            }
        }

    }

    pub fn as_model(&mut self, engine: &EngineData) -> &InstancedModel {

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

        if self.model.is_some() {
            self.model.as_mut().unwrap().update_instances(&engine.get_device(), &instances);
        } else {
            self.model = Some(instanced_model::InstancedModel::new(
                &engine.get_device(), 
                "assets/cube.obj", 
                instances,
            )); 
        }

        self.model.as_ref().unwrap()
    }

    pub fn to_chunk_coords(&self, pos : [f32; 3]) -> [i32; 2] {
        [(pos[0] / (self.chunk_size * self.tile_size)) as i32, 
        (pos[2] / (self.chunk_size * self.tile_size)) as i32]
    }
    
    pub fn to_world_coords(&self, chunk_coords : [i32; 2]) -> [f32; 3] {
        [chunk_coords[0] as f32 * self.chunk_size * self.tile_size, 
        0.0, 
        chunk_coords[1] as f32 * self.chunk_size * self.tile_size]
    }

    pub fn get_chunks_in_view(&self, pos : i32) -> std::ops::Range<i32> {
        (pos - self.chunks_in_view)..(pos + self.chunks_in_view)
    }

    pub fn map_height_function(&mut self, x : f32, y : f32) -> f32 {
        self.aesthetics.get_height_from(x, y)
    }

    pub fn map_color_function(&self, x : f32, y : f32, height : f32) -> f32 {

        let r = self.aesthetics.get_material_from(x, y, height); 

        if r < 0.4 {
            return 0.0;
        } else if r < 0.5 {
            return 1.0;
        } else if r < 0.6 {
            return 2.0;
        } else {
            return 3.0;
        }
        
    }

}