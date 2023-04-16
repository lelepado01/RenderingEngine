use crate::engine::utils::array_math::Add;

use super::map_height_function;

pub struct Tile {
    pub position: [f32; 3],
    pub material: f32,
}

pub struct TileChunk {
    pub chunk_coords: [i32; 2],
    pub tiles: Vec<Tile>,
    pub center: [f32; 3],
    size: f32,
}


impl TileChunk {
    pub fn new(x : i32, y : i32, tile_size : f32, chunk_size : f32) -> Self {

        let start_pos = [
            x as f32 * tile_size * chunk_size, 
            0.0, 
            y as f32 * tile_size * chunk_size
        ];

        let mut chunk = TileChunk {
            chunk_coords: [x, y],
            tiles: Vec::new(),
            center: start_pos.add([chunk_size / 2.0, 0.0, chunk_size / 2.0]),
            size: chunk_size,
        }; 

        for i in 0..chunk.size as i32 {
            for j in 0..chunk.size as i32 {
                let x = start_pos[0] + i as f32 * tile_size;
                let z = start_pos[2] + j as f32 * tile_size;

                let height = map_height_function(x, z);
                let mat_id : f32 = (i % 3) as f32;
                chunk.tiles.push(Tile { position: [x, height, z], material: mat_id });
            }
        }

        chunk
    }
}