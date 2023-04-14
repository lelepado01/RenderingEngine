use super::Tile;
use crate::engine::utils::array_math::Add;

pub struct TileChunk {
    pub chunk_coords: [i32; 2],
    pub tiles: Vec<Tile>,
    pub center: [f32; 3],
    pub size: f32,
}


impl TileChunk {
    pub fn new(x : i32, y : i32, size : f32, start_pos : [f32; 3]) -> Self {
        let mut chunk = TileChunk {
            chunk_coords: [x, y],
            tiles: Vec::new(),
            center: start_pos.add([size / 2.0, 0.0, size / 2.0]),
            size,
        }; 

        for i in 0..size as i32 {
            for j in 0..size as i32 {
                let height = (start_pos[0] + i as f32 * 0.2).sin() * 3.0 * (start_pos[2] + j as f32 * 0.1).cos() * 3.0;
                let mat_id : f32 = (i % 3) as f32;
                let tile = Tile {
                    position: [start_pos[0] + 2.0 * i as f32, height, start_pos[2] + 2.0* j as f32],
                    material: mat_id,
                };

                chunk.tiles.push(tile);
            }
        }

        chunk
    }
}