use image::GenericImageView;
use noise::{Fbm, OpenSimplex, utils::{PlaneMapBuilder, NoiseMapBuilder}};
use tiff::decoder::{DecodingResult, DecodingBuffer};

const MAP_HEIGHT_CONTRIBUTION : f32 = 0.5;

pub struct MapAestheticsParams {
    height_map: DecodingResult,
    image: image::DynamicImage,
}

impl MapAestheticsParams {
    pub fn new() -> Self {
        let mut decoder = tiff::decoder::Decoder::new(
            std::fs::File::open("assets/heightmap.tif").unwrap()
        ).unwrap(); 

        if !std::path::Path::new("assets/material_noise_map.png").exists(){
            println!("Generating material noise map");
            let fbm = Fbm::<OpenSimplex>::new(0);
            PlaneMapBuilder::<_, 2>::new(&fbm)
              .set_size(super::MAP_SIZE_X, super::MAP_SIZE_Y)
              .set_x_bounds(-5.0, 5.0)
              .set_y_bounds(-5.0, 5.0)
              .build()
            .write_to_file("../assets/material_noise_map.png");  
        }

        let image = image::open("assets/material_noise_map.png").expect("Failed to load image");
        
        MapAestheticsParams {
            height_map: decoder.read_image().unwrap(),
            image,
        }
    }

    pub fn get_height_from(&mut self, x : f32, y : f32) -> f32 {
        let val = match &self.height_map.as_buffer(0) {
            DecodingBuffer::U8(m) => m[(x as usize) + (y as usize) * super::MAP_SIZE_Y] as f32,// / 255.0,
            DecodingBuffer::U16(m) => m[(x as usize) + (y as usize) * super::MAP_SIZE_Y] as f32,// / 65535.0,
            DecodingBuffer::U32(m) => m[(x as usize) + (y as usize) * super::MAP_SIZE_Y] as f32,// / 4294967295.0,
            DecodingBuffer::U64(m) => m[(x as usize) + (y as usize) * super::MAP_SIZE_Y] as f32,// / 18446744073709551615.0,
            DecodingBuffer::I8(m) => m[(x as usize) + (y as usize) * super::MAP_SIZE_Y] as f32,// / 127.0,
            DecodingBuffer::I16(m) => m[(x as usize) + (y as usize) * super::MAP_SIZE_Y] as f32,// / 32767.0,
            DecodingBuffer::I32(m) => m[(x as usize) + (y as usize) * super::MAP_SIZE_Y] as f32, //  / 2147483647.0,
            DecodingBuffer::I64(m) => m[(x as usize) + (y as usize) * super::MAP_SIZE_Y] as f32, // / 9223372036854775807.0,
            DecodingBuffer::F32(m) => m[(x as usize) + (y as usize) * super::MAP_SIZE_Y],
            DecodingBuffer::F64(m) => m[(x as usize) + (y as usize) * super::MAP_SIZE_Y] as f32,
        }; 

        ((val -2961.0) / 54606.0 ) * super::MAP_HEIGHT
    }

    pub fn get_material_from(&self, x : f32, y : f32, height : f32) -> f32 {
        if x < 0.0 || x > super::MAP_SIZE_X as f32 || y < 0.0 || y > super::MAP_SIZE_Y as f32 {
            return 0.0;
        }
        
        let pixel = self.image.get_pixel(x as u32, y as u32);
        let r = pixel[0] as f32 / 255.0;

        let height = height / super::MAP_HEIGHT;
        r * (1.0 - MAP_HEIGHT_CONTRIBUTION) + height * MAP_HEIGHT_CONTRIBUTION
    }
}