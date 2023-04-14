use crate::engine::{models::{vertices::instance_data::PositionInstanceData, instanced_model::{InstancedModel, self}}, engine::EngineData};
mod chunk;
use chunk::TileChunk;

use crate::physics::get_distance; 
use self::chunk::{to_chunk_coords, to_world_coords, Tile};

pub struct TileMap {
    pub tiles: Vec<Tile>,
    pub chunks: Vec<TileChunk>,
    max_distance: f32,
    chunk_size : f32,
    model: std::option::Option<InstancedModel>,
}

impl TileMap {
    pub fn new() -> Self {

        let mut tilemap= TileMap {
            tiles: Vec::new(),
            chunks: Vec::new(),
            max_distance: 100.0,
            chunk_size: 30.0,
            model: None,
        };
        
        for i in 0..10 {
            for j in 0..10 {
                let chunk = TileChunk::new(i, j, tilemap.chunk_size, [i as f32 * 2.0 * tilemap.chunk_size, 0.0, j as f32 * 2.0 * tilemap.chunk_size]);
                tilemap.chunks.push(chunk);
            }
        }

        tilemap
    }

    pub fn update(&mut self, player_position : &[f32; 3]) {

        // remove chunks that are too far away
        self.chunks.retain(|x| { 
            get_distance(&x.center, player_position) < self.max_distance 
        });

        let cam_chunk_pos = to_chunk_coords(*player_position, self.chunk_size); 

        for i in cam_chunk_pos[0]..(cam_chunk_pos[0]+10) {
            for j in cam_chunk_pos[1]..(cam_chunk_pos[1]+10) {
                
                if self.chunks.iter().any(|x| x.chunk_coords == [i, j]) {
                    continue;
                }
                
                let chunk_pos = to_world_coords([i, j], 30.0); 
                let dist = get_distance(&chunk_pos, player_position);
                if dist < self.max_distance {
                    let chunk = TileChunk::new(i, j, self.chunk_size, chunk_pos);
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
}