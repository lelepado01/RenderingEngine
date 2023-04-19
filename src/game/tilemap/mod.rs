use crate::engine::{models::{vertices::instance_data::PositionInstanceData, instanced_model::{InstancedModel, self}}, engine::EngineData, utils::array_math::ScalarMul};
mod chunk;
use chunk::TileChunk;
use tiff::decoder::{DecodingBuffer, Limits, DecodingResult};
use noise::{NoiseFn, Perlin, OpenSimplex, utils::PlaneMapBuilder, Fbm};

use crate::physics::get_distance; 
use image::GenericImageView;

pub struct TileMap {
    pub chunks: Vec<TileChunk>,
    max_distance: f32,
    chunks_in_view : i32,
    chunk_size : f32,
    tile_size : f32,
    model: std::option::Option<InstancedModel>,

    height_map: DecodingResult,
    noise: OpenSimplex,
    image: image::DynamicImage,
}

impl TileMap {
    pub fn new() -> Self {

        let mut decoder = tiff::decoder::Decoder::new(
            std::fs::File::open("assets/heightmap.tif").unwrap()
        )
            .unwrap()
            .with_limits(Limits::default()); 

        let noise: OpenSimplex = OpenSimplex::new(1);
        let fbm = Fbm::<OpenSimplex>::new(0);

        use noise::utils::NoiseMapBuilder;
        PlaneMapBuilder::<_, 2>::new(&fbm)
          .set_size(1000, 1000)
          .set_x_bounds(-5.0, 5.0)
          .set_y_bounds(-5.0, 5.0)
          .build()
        .write_to_file("../assets/material_noise_map.png");  

        let image = image::open("assets/material_noise_map.png").expect("Failed to load image");

        
        let mut tilemap= TileMap {
            chunks: Vec::new(),
            max_distance: 0.0,
            chunks_in_view: 5,
            chunk_size: 30.0,
            tile_size: 2.0,
            model: None,
            height_map: decoder.read_image().unwrap(),
            noise,
            image,
        };

        tilemap.max_distance = tilemap.chunks_in_view as f32 * tilemap.chunk_size * tilemap.tile_size;
        
        for i in tilemap.get_chunks_in_view(0) {
            for j in tilemap.get_chunks_in_view(0) {
                let chunk = TileChunk::new(i, j, &mut tilemap);
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
                    let chunk = TileChunk::new(i, j, self);
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
        // let x_dim  = 5400; 
        let y_dim = 2700; 
        let val = match &self.height_map.as_buffer(0) {
            DecodingBuffer::U8(m) => m[(x as usize) + (y as usize) * y_dim] as f32,// / 255.0,
            DecodingBuffer::U16(m) => m[(x as usize) + (y as usize) * y_dim] as f32,// / 65535.0,
            DecodingBuffer::U32(m) => m[(x as usize) + (y as usize) * y_dim] as f32,// / 4294967295.0,
            DecodingBuffer::U64(m) => m[(x as usize) + (y as usize) * y_dim] as f32,// / 18446744073709551615.0,
            DecodingBuffer::I8(m) => m[(x as usize) + (y as usize) * y_dim] as f32,// / 127.0,
            DecodingBuffer::I16(m) => m[(x as usize) + (y as usize) * y_dim] as f32,// / 32767.0,
            DecodingBuffer::I32(m) => m[(x as usize) + (y as usize) * y_dim] as f32, //  / 2147483647.0,
            DecodingBuffer::I64(m) => m[(x as usize) + (y as usize) * y_dim] as f32, // / 9223372036854775807.0,
            DecodingBuffer::F32(m) => m[(x as usize) + (y as usize) * y_dim],
            DecodingBuffer::F64(m) => m[(x as usize) + (y as usize) * y_dim] as f32,
        }; 

        ((val -2961.0) / 54606.0 ) * 250.0
    }

    pub fn map_color_function(&self, x : f32, y : f32, z : f32) -> f32 {

        let pixel = self.image.get_pixel(x as u32, y as u32);
        let r = pixel[0] as f32 / 255.0;

        if r < 0.1 {
            return 0.0;
        } else if r < 0.2 {
            return 1.0;
        } else if r < 0.3 {
            return 2.0;
        } else {
            return 3.0;
        }
        
    }

}